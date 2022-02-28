use crate::entity::prelude::{
    IhAreas, IhFacilities, IhHouseFacilities, IhHouseImages, IhHouses, IhOrders, IhUsers,
};
use crate::entity::{ih_house_facilities, ih_house_images, ih_houses, ih_orders};
use crate::utils::constants;
use crate::utils::{
    constants::OrderStatus,
    jwt::AuthUser,
    response_codes::{ResponseInfo, RetError},
};
use axum::{
    extract::{Extension, Multipart, Path, Query},
    Json,
};
use chrono::prelude::*;
use redis::{AsyncCommands, Client};
use sea_orm::ActiveModelTrait;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// -------------request-------------------
#[derive(Debug, Deserialize)]
pub struct OrderReq {
    house_id: String,
    start_date: String,
    end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct HandleReq {
    action: String,
    comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PayReq {
    order_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentReq {
    comment: String,
}

// -------------response-------------------
#[derive(Debug, Serialize)]
pub struct GetOrderRes {
    order_id: i32,
    status: String,
    status_info: String,
    img_url: String,
    house_id: i32,
    title: String,
    ctime: String,
    start_date: String,
    end_date: String,
    amount: i32,
    days: i32,
    comment: String,
}

// 创建订单
pub async fn create_order(
    auth_user: AuthUser,
    Json(payload): Json<OrderReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<String>>, RetError> {
    let house_id: i32 = payload.house_id.parse().unwrap();
    let start_date = payload.start_date;
    let end_date = payload.end_date;
    let start_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();
    let days = (end_date - start_date).num_days() as i32;
    // 查看该房屋在该时间段是否有预定
    if IhOrders::find()
        .filter(ih_orders::Column::HouseId.eq(house_id))
        .filter(ih_orders::Column::StartDate.lte(end_date))
        .filter(ih_orders::Column::EndDate.gte(start_date))
        .filter(ih_orders::Column::Status.is_in([
            constants::OrderStatus::WaitAccept.get_info(),
            constants::OrderStatus::WaitPayment.get_info(),
        ]))
        .one(&db)
        .await
        .unwrap()
        .is_some()
    {
        return Err(RetError::PARAMERR("该时间段已被订购".to_string()));
    }
    // 乐观锁
    for _ in 0..3 {
        let house = IhHouses::find_by_id(house_id).one(&db).await.unwrap();
        let house = house
            .ok_or(RetError::PARAMERR("house_id不存在".to_string()))
            .unwrap();
        if auth_user.user_id == house.user_id.unwrap() {
            return Err(RetError::PARAMERR("不能预定自己的房屋".to_string()));
        }
        let old_order_count = house.order_count.unwrap_or(0);
        let amount = days * house.price.unwrap();
        // 创建订单
        let order = ih_orders::ActiveModel {
            user_id: Set(Some(auth_user.user_id)),
            house_id: Set(Some(house_id)),
            start_date: Set(Some(start_date)),
            end_date: Set(Some(end_date)),
            days: Set(Some(days)),
            price: Set(Some(house.price.unwrap())),
            amount: Set(Some(amount)),
            status: Set(Some(
                constants::OrderStatus::WaitAccept.get_info().to_string(),
            )),
            ..Default::default()
        };
        IhOrders::insert(order).exec(&db).await.unwrap();
        // 更新订单数量
        let mut house: ih_houses::ActiveModel = house.into();
        house.order_count = Set(Some(old_order_count + 1));
        house.update(&db).await.unwrap();
        return Ok(Json(ResponseInfo {
            errno: "0".to_string(),
            data: "0".to_string(),
        }));
    }
    Err(RetError::DATAERR("该房屋已被预定".to_string()))
}

// 查询订单
pub async fn get_orders(
    auth_user: AuthUser,
    Query(params): Query<HashMap<String, String>>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<Vec<GetOrderRes>>>, RetError> {
    let mut orders: Vec<GetOrderRes> = Vec::new();
    // 查询订单
    let order_models = match params.get("role") {
        Some(r) if r.as_str() == "lorder" => {
            let houses = IhHouses::find()
                .filter(ih_houses::Column::UserId.eq(auth_user.user_id))
                .all(&db)
                .await
                .unwrap();
            let house_ids = houses.iter().map(|h| h.id);
            IhOrders::find()
                .filter(ih_orders::Column::HouseId.is_in(house_ids))
                .all(&db)
                .await
                .unwrap()
        }
        _ => IhOrders::find()
            .filter(ih_orders::Column::UserId.eq(auth_user.user_id))
            .all(&db)
            .await
            .unwrap(),
    };
    for order in order_models {
        // 查询房屋
        let house = IhHouses::find_by_id(order.house_id.unwrap())
            .one(&db)
            .await
            .unwrap();
        if house.is_none() {
            continue;
        }
        let house = house.unwrap();
        // // 生成订单信息
        orders.push(GetOrderRes {
            order_id: order.id,
            status: order.status.clone().unwrap(),
            status_info: OrderStatus::new(&order.status.unwrap())
                .get_desc()
                .to_string(),
            img_url: house.default_image_url.unwrap(),
            house_id: order.house_id.unwrap(),
            title: house.title.unwrap(),
            ctime: order
                .created_date
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            start_date: order.start_date.unwrap().format("%Y-%m-%d").to_string(),
            end_date: order.end_date.unwrap().format("%Y-%m-%d").to_string(),
            amount: order.amount.unwrap(),
            days: order.days.unwrap(),
            comment: order.comment.unwrap_or("".to_string()),
        });
    }
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: orders,
    }))
}

// 接收/拒绝订单
pub async fn handle_order(
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
    Json(payload): Json<HandleReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<String>>, RetError> {
    // 获取对应的操作和评论信息
    let action = payload.action;
    let comment = payload.comment.unwrap_or("".to_string());
    // 获取订单
    let order = IhOrders::find_by_id(order_id).one(&db).await.unwrap();
    let order = order
        .ok_or(RetError::PARAMERR("order_id不存在".to_string()))
        .unwrap();
    // 校验
    if order.status.as_ref().unwrap() != OrderStatus::WaitAccept.get_info() {
        return Err(RetError::DATAERR("订单状态不为`待接单`".to_string()));
    }
    let house = IhHouses::find_by_id(order.house_id.unwrap())
        .one(&db)
        .await
        .unwrap();
    let house = house
        .ok_or(RetError::PARAMERR("house_id不存在".to_string()))
        .unwrap();
    if house.user_id.unwrap() != auth_user.user_id {
        return Err(RetError::DATAERR("订单房屋不属于当前用户".to_string()));
    }
    let status = match action.as_str() {
        "accept" => OrderStatus::WaitPayment.get_info(),
        "reject" => OrderStatus::Rejected.get_info(),
        _ => return Err(RetError::DATAERR("无效的操作".to_string())),
    };
    // 更新
    let mut order: ih_orders::ActiveModel = order.into();
    order.comment = Set(Some(comment));
    order.status = Set(Some(status.to_string()));
    order.update(&db).await.unwrap();
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: "".to_string(),
    }))
}

// 支付订单
pub async fn pay_order(
    auth_user: AuthUser,
    Json(payload): Json<PayReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<String>>, RetError> {
    let order_id: i32 = payload.order_id.parse().unwrap();
    // 获取订单
    let order = IhOrders::find_by_id(order_id).one(&db).await.unwrap();
    let order = order
        .ok_or(RetError::PARAMERR("order_id不存在".to_string()))
        .unwrap();
    // 校验状态
    if order.status.as_ref().unwrap() != OrderStatus::WaitPayment.get_info() {
        return Err(RetError::DATAERR("订单状态不为`待支付`".to_string()));
    }
    // 校验用户
    if order.user_id.unwrap() != auth_user.user_id {
        return Err(RetError::DATAERR("订单不属于当前用户".to_string()));
    }
    // 更新状态
    let mut order: ih_orders::ActiveModel = order.into();
    order.status = Set(Some(OrderStatus::WaitComment.get_info().to_string()));
    order.update(&db).await.unwrap();
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: "".to_string(),
    }))
}

// 评论订单
pub async fn comment_order(
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
    Json(payload): Json<CommentReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<String>>, RetError> {
    let comment = payload.comment;
    // 获取订单
    let order = IhOrders::find_by_id(order_id).one(&db).await.unwrap();
    let order = order
        .ok_or(RetError::PARAMERR("order_id不存在".to_string()))
        .unwrap();
    // 校验状态
    if order.status.as_ref().unwrap() != OrderStatus::WaitComment.get_info() {
        return Err(RetError::DATAERR("订单状态不为`待评论`".to_string()));
    }
    // 校验用户
    if order.user_id.unwrap() != auth_user.user_id {
        return Err(RetError::DATAERR("订单不属于当前用户".to_string()));
    }
    // 更新状态
    let mut order: ih_orders::ActiveModel = order.into();
    order.status = Set(Some(OrderStatus::Completed.get_info().to_string()));
    order.comment = Set(Some(comment));
    order.update(&db).await.unwrap();
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: "".to_string(),
    }))
}

// 取消订单
pub async fn cancel_order(
    auth_user: AuthUser,
    Path(order_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<String>>, RetError> {
    // 获取订单
    let order = IhOrders::find_by_id(order_id).one(&db).await.unwrap();
    let order = order
        .ok_or(RetError::PARAMERR("order_id不存在".to_string()))
        .unwrap();
    // 校验状态
    if order.status.as_ref().unwrap() != OrderStatus::WaitAccept.get_info()
        || order.status.as_ref().unwrap() != OrderStatus::WaitComment.get_info()
    {
        return Err(RetError::DATAERR("订单状态不为`待接单`或`待支付`".to_string()));
    }
    // 校验用户
    if order.user_id.unwrap() != auth_user.user_id {
        return Err(RetError::DATAERR("订单不属于当前用户".to_string()));
    }
    // 更新状态
    let mut order: ih_orders::ActiveModel = order.into();
    order.status = Set(Some(OrderStatus::Completed.get_info().to_string()));
    order.update(&db).await.unwrap();
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: "".to_string(),
    }))
}
