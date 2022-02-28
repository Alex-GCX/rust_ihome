use crate::entity::prelude::{
    IhAreas, IhFacilities, IhHouseFacilities, IhHouseImages, IhHouses, IhOrders, IhUsers,
};
use crate::entity::{ih_house_facilities, ih_house_images, ih_houses, ih_orders};
use crate::utils::{
    constants,
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
use std::{
    fs::{remove_file, File},
    io::Write,
};

// -------------request-------------------
#[derive(Debug, Deserialize)]
pub struct HouseReq {
    title: String,
    price: String,
    area_id: String,
    address: String,
    room_count: String,
    acreage: String,
    unit: String,
    capacity: String,
    beds: String,
    deposit: String,
    min_days: String,
    max_days: String,
    facilities: Vec<String>,
    house_id: Option<String>,
}

// -------------response-------------------
#[derive(Debug, Serialize)]
pub struct PublishHouseRes {
    house_id: i32,
}

#[derive(Debug, Serialize)]
pub struct HouseImageRes {
    url: String,
}

#[derive(Debug, Serialize)]
pub struct UserHouseRes {
    house_id: i32,
    title: String,
    area_name: String,
    price: i32,
    created_date: String,
    img_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentRes {
    user_name: String,
    comment_date: String,
    comment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HouseInfo {
    img_urls: Vec<String>,
    title: String,
    price: i32,
    owner_id: i32,
    owner_img_url: String,
    owner_name: String,
    address: String,
    room_count: i32,
    acreage: i32,
    unit: String,
    capacity: i32,
    beds: String,
    deposit: i32,
    min_days: i32,
    max_days: i32,
    facilities: Vec<i32>,
    comments: Vec<CommentRes>,
}

#[derive(Debug, Serialize)]
pub struct HouseInfoRes {
    user_id: i32,
    house: HouseInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexHouseRes {
    id: i32,
    title: String,
    img_url: String,
    price: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchHouse {
    house_id: i32,
    title: String,
    img_url: String,
    owner_img_url: String,
    price: i32,
    room_count: i32,
    order_count: i32,
    address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchHouseRes {
    house_info: Vec<SearchHouse>,
    current_page: usize,
    total_page: usize,
}

#[derive(Debug, Serialize)]
pub struct BookingHouseRes {
    house_id: i32,
    title: String,
    img_url: String,
    price: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexHouseCache {
    data: Vec<IndexHouseRes>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexAreasCache {
    data: HashMap<i32, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchCache {
    data: SearchHouseRes,
}
// 获取区域信息
pub async fn get_areas(
    Extension(db): Extension<DatabaseConnection>,
    Extension(redis_client): Extension<Client>,
) -> Json<ResponseInfo<HashMap<i32, String>>> {
    let mut con = redis_client.get_async_connection().await.unwrap();
    let mut data: HashMap<i32, String> = HashMap::new();
    let areas = con.get("ih_areas").await.unwrap_or("".to_string());
    if areas != "".to_string() {
        // 缓存存在
        let cache: IndexAreasCache = serde_json::from_str(&areas).unwrap();
        data = cache.data;
    } else {
        let rows = IhAreas::find().all(&db).await.unwrap();
        for row in rows {
            data.insert(row.id, row.name.unwrap());
        }
        con.set_ex::<&str, String, i32>(
            "ih_areas",
            serde_json::to_string(&IndexAreasCache { data: data.clone() }).unwrap(),
            constants::AREA_REDIS_EXPIRES,
        )
        .await
        .unwrap();
    }

    Json(ResponseInfo {
        errno: "0".to_string(),
        data,
    })
}

// 创建房屋信息
pub async fn publish_house(
    auth_user: AuthUser,
    Json(payload): Json<HouseReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<PublishHouseRes>>, RetError> {
    let title = payload.title;
    let price = payload.price.parse::<i32>().unwrap();
    let area_id = payload.area_id.parse::<i32>().unwrap();
    let address = payload.address;
    let room_count = payload.room_count.parse::<i32>().unwrap();
    let acreage = payload.acreage.parse::<i32>().unwrap();
    let unit = payload.unit;
    let capacity = payload.capacity.parse::<i32>().unwrap();
    let beds = payload.beds;
    let deposit = payload.deposit.parse::<i32>().unwrap();
    let min_days = payload.min_days.parse::<i32>().unwrap();
    let max_days = payload.max_days.parse::<i32>().unwrap();
    let facilities = payload.facilities;
    // area_id是否存在
    let area = IhAreas::find_by_id(area_id).one(&db).await.unwrap();
    if area == None {
        return Err(RetError::DATAERR("area_id不存在".to_string()));
    }
    // 最小最大入住天数
    if min_days > max_days {
        return Err(RetError::DATAERR(
            "最少入住天数不能大于最大入住天数".to_string(),
        ));
    }
    // 存表
    // 创建房子
    let house = ih_houses::ActiveModel {
        user_id: Set(Some(auth_user.user_id)),
        area_id: Set(Some(area_id)),
        title: Set(Some(title.clone())),
        price: Set(Some(price)),
        address: Set(Some(address.clone())),
        room_count: Set(Some(room_count)),
        acreage: Set(Some(acreage)),
        unit: Set(Some(unit.clone())),
        capacity: Set(Some(capacity)),
        beds: Set(Some(beds.clone())),
        deposit: Set(Some(deposit)),
        min_days: Set(Some(min_days)),
        max_days: Set(Some(max_days)),
        ..Default::default()
    };
    let res = IhHouses::insert(house).exec(&db).await.unwrap();
    let house_id = res.last_insert_id;
    for fac in facilities {
        // 校验fac
        let fac_id = fac.parse::<i32>().unwrap();
        let fac_model = IhFacilities::find_by_id(fac_id).one(&db).await.unwrap();
        if fac_model == None {
            continue;
        }
        //
        let house_fac = ih_house_facilities::ActiveModel {
            house_id: Set(Some(house_id)),
            facility_id: Set(Some(fac_id)),
            ..Default::default()
        };
        IhHouseFacilities::insert(house_fac)
            .exec(&db)
            .await
            .unwrap();
    }

    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: PublishHouseRes { house_id },
    }))
}

// 更新房屋信息
pub async fn update_house(
    _auth_user: AuthUser,
    Json(payload): Json<HouseReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<PublishHouseRes>>, RetError> {
    let house_id = payload
        .house_id
        .unwrap_or("0".to_string())
        .parse::<i32>()
        .unwrap();
    let title = payload.title;
    let price: i32 = payload.price.parse::<i32>().unwrap();
    let area_id = payload.area_id.parse::<i32>().unwrap();
    let address = payload.address;
    let room_count = payload.room_count.parse::<i32>().unwrap();
    let acreage = payload.acreage.parse::<i32>().unwrap();
    let unit = payload.unit;
    let capacity = payload.capacity.parse::<i32>().unwrap();
    let beds = payload.beds;
    let deposit = payload.deposit.parse::<i32>().unwrap();
    let min_days = payload.min_days.parse::<i32>().unwrap();
    let max_days = payload.max_days.parse::<i32>().unwrap();
    let facilities = payload.facilities;
    // house_id是否存在
    let house = IhHouses::find_by_id(house_id).one(&db).await.unwrap();
    if house == None {
        return Err(RetError::DATAERR("house_id不存在".to_string()));
    }
    // area_id是否存在
    let area = IhAreas::find_by_id(area_id).one(&db).await.unwrap();
    if area == None {
        return Err(RetError::DATAERR("area_id不存在".to_string()));
    }
    // 最小最大入住天数
    if min_days > max_days {
        return Err(RetError::DATAERR(
            "最少入住天数不能大于最大入住天数".to_string(),
        ));
    }
    // 存表
    // 更新房子
    let mut house: ih_houses::ActiveModel = house.unwrap().into();
    house.area_id = Set(Some(area_id));
    house.title = Set(Some(title.clone()));
    house.price = Set(Some(price));
    house.address = Set(Some(address.clone()));
    house.room_count = Set(Some(room_count));
    house.acreage = Set(Some(acreage));
    house.unit = Set(Some(unit.clone()));
    house.capacity = Set(Some(capacity));
    house.beds = Set(Some(beds.clone()));
    house.deposit = Set(Some(deposit));
    house.min_days = Set(Some(min_days));
    house.max_days = Set(Some(max_days));
    house.update(&db).await.unwrap();
    // 删除原房屋与设施的绑定
    IhHouseFacilities::delete_many()
        .filter(ih_house_facilities::Column::HouseId.eq(house_id))
        .exec(&db)
        .await
        .unwrap();
    // 创建新的绑定
    for fac in facilities {
        // 校验fac
        let fac_id = fac.parse::<i32>().unwrap();
        let fac_model = IhFacilities::find_by_id(fac_id).one(&db).await.unwrap();
        if fac_model == None {
            continue;
        }
        //
        let house_fac = ih_house_facilities::ActiveModel {
            house_id: Set(Some(house_id)),
            facility_id: Set(Some(fac_id)),
            ..Default::default()
        };
        IhHouseFacilities::insert(house_fac)
            .exec(&db)
            .await
            .unwrap();
    }

    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: PublishHouseRes { house_id },
    }))
}

// 添加房屋图片
pub async fn set_house_image(
    _auth_user: AuthUser,
    mut multipart: Multipart,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<HouseImageRes>>, RetError> {
    // 获取时间戳
    let timestamp = Local::now().timestamp_millis().to_string();
    let url = format!("static/images/houses/{}.jpg", timestamp);
    let mut house_id = 0;
    // 读取传入的文件
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        if name == "house_image" {
            // 保存文件
            let data = field.bytes().await.unwrap();
            let mut file = File::create(&url).unwrap();
            file.write(&data).unwrap();
        } else {
            house_id = field.text().await.unwrap().parse::<i32>().unwrap_or(0);
        }
    }
    // 判断house是否存在
    let house = IhHouses::find_by_id(house_id).one(&db).await.unwrap();
    let house = match house {
        Some(h) => h,
        None => {
            match remove_file(&url) {
                Ok(_) => {}
                Err(_) => return Err(RetError::DBERR("删除文件失败".to_string())),
            }
            return Err(RetError::DATAERR("house不存在".to_string()));
        }
    };
    // 新增房屋图片
    let house_image = ih_house_images::ActiveModel {
        house_id: Set(Some(house.id)),
        image_url: Set(Some(url.clone())),
        ..Default::default()
    };
    IhHouseImages::insert(house_image).exec(&db).await.unwrap();
    // 更新
    let mut house: ih_houses::ActiveModel = house.into();
    house.default_image_url = Set(Some(url.clone()));
    house.update(&db).await.unwrap();

    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: HouseImageRes { url },
    }))
}

// 获取我的房源
pub async fn get_user_house(
    auth_user: AuthUser,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<Vec<UserHouseRes>>>, RetError> {
    let mut houses: Vec<UserHouseRes> = Vec::new();
    // 查询房屋
    let rows = IhHouses::find()
        .filter(ih_houses::Column::UserId.eq(auth_user.user_id))
        .all(&db)
        .await
        .unwrap();
    for row in rows {
        // 获取区域名称
        let area = IhAreas::find_by_id(row.area_id.unwrap())
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        // 构造返回的house信息
        let house = UserHouseRes {
            house_id: row.id,
            title: row.title.unwrap_or("".to_string()),
            area_name: area.name.unwrap_or("".to_string()),
            price: row.price.unwrap(),
            created_date: row
                .created_date
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            img_url: row.default_image_url.unwrap_or("".to_string()),
        };
        houses.push(house);
    }
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: houses,
    }))
}

// 获取房屋详情
pub async fn get_house_info(
    auth_user: AuthUser,
    Path(house_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(redis_client): Extension<Client>,
) -> Result<Json<ResponseInfo<HouseInfoRes>>, RetError> {
    let mut con = redis_client.get_async_connection().await.unwrap();
    // let mut house_info: HouseInfo = HouseInfo::default();
    let house_info: HouseInfo;
    // 从缓存中获取
    let cache = con
        .hget("ih_house_info", house_id)
        .await
        .unwrap_or("".to_string());
    if cache != "".to_string() {
        // 缓存中存在
        house_info = serde_json::from_str(&cache).unwrap();
    } else {
        let house = IhHouses::find_by_id(house_id).one(&db).await.unwrap();
        let house = house
            .ok_or(RetError::DATAERR("house_id不存在".to_string()))
            .unwrap();
        // 获取images
        let images = IhHouseImages::find()
            .filter(ih_house_images::Column::HouseId.eq(house.id))
            .all(&db)
            .await
            .unwrap();
        let mut img_urls: Vec<String> = Vec::new();
        for img in images {
            img_urls.push(img.image_url.unwrap_or("".to_string()));
        }
        // 获取owner
        let owner = IhUsers::find_by_id(house.user_id.unwrap())
            .one(&db)
            .await
            .unwrap();
        let owner = owner
            .ok_or(RetError::DATAERR("获取owner失败".to_string()))
            .unwrap();
        // 获取facilities
        let facilities_model = IhHouseFacilities::find()
            .filter(ih_house_facilities::Column::HouseId.eq(house.id))
            .all(&db)
            .await
            .unwrap();
        let mut facilities: Vec<i32> = Vec::new();
        for fac in facilities_model {
            facilities.push(fac.facility_id.unwrap());
        }
        // 获取comments
        let orders = IhOrders::find()
            .filter(ih_orders::Column::HouseId.eq(house.id))
            .filter(ih_orders::Column::Comment.is_not_null())
            .limit(constants::COMMENT_DISPLAY_COUNTS)
            .all(&db)
            .await
            .unwrap();
        let mut comments: Vec<CommentRes> = Vec::new();
        for order in orders {
            let user = IhUsers::find_by_id(order.user_id.unwrap())
                .one(&db)
                .await
                .unwrap()
                .unwrap();
            comments.push(CommentRes {
                user_name: user.name.unwrap(),
                comment_date: order
                    .updated_date
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                comment: order.comment.unwrap_or("".to_string()),
            });
        }

        house_info = HouseInfo {
            img_urls,
            title: house.title.unwrap(),
            price: house.price.unwrap(),
            owner_id: house.user_id.unwrap(),
            owner_img_url: owner.image_url.unwrap_or("".to_string()),
            owner_name: owner.name.unwrap(),
            address: house.address.unwrap(),
            room_count: house.room_count.unwrap(),
            acreage: house.acreage.unwrap(),
            unit: house.unit.unwrap(),
            capacity: house.capacity.unwrap(),
            beds: house.beds.unwrap(),
            deposit: house.deposit.unwrap(),
            min_days: house.min_days.unwrap(),
            max_days: house.max_days.unwrap(),
            facilities,
            comments,
        };
        con.hset::<&str, i32, String, i32>(
            "ih_house_info",
            house_id,
            serde_json::to_string(&house_info.clone()).unwrap(),
        )
        .await
        .unwrap();
        con.expire::<&str, i32>("ih_house_info", constants::HOUSE_REDIS_EXPIRES)
            .await
            .unwrap();
    }

    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: HouseInfoRes {
            user_id: auth_user.user_id,
            house: house_info,
        },
    }))
}

// 获取首页房屋信息
pub async fn get_index_houses(
    _auth_user: AuthUser,
    Extension(db): Extension<DatabaseConnection>,
    Extension(redis_client): Extension<Client>,
) -> Result<Json<ResponseInfo<Vec<IndexHouseRes>>>, RetError> {
    let mut redis_con = redis_client.get_async_connection().await.unwrap();
    // 从缓存中获取
    let res = redis_con
        .get("ih_index_house")
        .await
        .unwrap_or("".to_string());
    let mut data: Vec<IndexHouseRes> = Vec::new();
    if res != "".to_string() {
        // 存在缓存
        let cache: IndexHouseCache = serde_json::from_str(&res).unwrap();
        data = cache.data;
    } else {
        let houses = IhHouses::find()
            .order_by_desc(ih_houses::Column::OrderCount)
            .limit(constants::INDEX_HOUSES_COUNT as u64)
            .all(&db)
            .await
            .unwrap();
        for house in houses {
            data.push(IndexHouseRes {
                id: house.id,
                title: house.title.unwrap(),
                img_url: house.default_image_url.unwrap_or("".to_string()),
                price: house.price.unwrap(),
            })
        }
        redis_con
            .set_ex::<&str, String, i32>(
                "ih_index_house",
                serde_json::to_string(&IndexHouseCache { data: data.clone() }).unwrap(),
                constants::INDEX_HOUSES_EXPIRES,
            )
            .await
            .unwrap();
    }
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data,
    }))
}

// 搜索房屋
pub async fn get_search_houses(
    _auth_user: AuthUser,
    Query(params): Query<HashMap<String, String>>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(redis_client): Extension<Client>,
) -> Result<Json<ResponseInfo<SearchHouseRes>>, RetError> {
    // 获取url参数
    let area_id = match params.get("aid") {
        Some(s) if s.as_str() != "" => s.as_str(),
        _ => "",
    };
    let start_date = match params.get("sd") {
        Some(s) if s.as_str() != "" => s.as_str(),
        _ => "1997-01-01",
    };
    let end_date = match params.get("ed") {
        Some(s) if s.as_str() != "" => s.as_str(),
        _ => "2999-01-01",
    };
    let page: usize = match params.get("page") {
        Some(s) if s.as_str() != "" => s.parse().unwrap_or(1),
        _ => 1,
    };
    let sorted_by = match params.get("sorted_by") {
        Some(s) if s.as_str() != "" => s.as_str(),
        _ => "new",
    };
    // 查询缓存
    let mut redis_con = redis_client.get_async_connection().await.unwrap();
    let redis_key = &format!(
        "search:{}:{}:{}:{}:{}",
        area_id, start_date, end_date, page, sorted_by
    );
    let data: SearchHouseRes;
    // 从缓存中获取
    let res = redis_con.get(redis_key).await.unwrap_or("".to_string());
    if res != "".to_string() {
        // 存在缓存
        let cache: SearchCache = serde_json::from_str(&res).unwrap();
        data = cache.data;
    } else {
        // 查询时间段内已经出租的房屋
        let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
        let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();
        let orders = IhOrders::find()
            .column(ih_orders::Column::HouseId)
            .filter(ih_orders::Column::StartDate.lte(end_date))
            .filter(ih_orders::Column::EndDate.gte(start_date))
            .filter(ih_orders::Column::Status.is_in([
                constants::OrderStatus::WaitAccept.get_info(),
                constants::OrderStatus::WaitPayment.get_info(),
            ]))
            .all(&db)
            .await
            .unwrap();
        let order_houses = orders.iter().map(|o| o.house_id.unwrap());
        // 查询未出租的房屋
        let mut query = IhHouses::find()
            .filter(ih_houses::Column::Id.is_not_in(order_houses))
            .filter(ih_houses::Column::IsDelete.eq(0));
        // 添加区域查询
        if area_id != "" {
            query = query.filter(ih_houses::Column::AreaId.eq(area_id.parse::<i32>().unwrap()));
        }
        // 添加排序
        let (order, by) = match sorted_by {
            "booking" => (ih_houses::Column::OrderCount, Order::Desc),
            "price-inc" => (ih_houses::Column::Price, Order::Asc),
            "price-des" => (ih_houses::Column::Price, Order::Desc),
            _ => (ih_houses::Column::CreatedDate, Order::Desc),
        };
        // 获取分页器
        let pagenate = query
            .order_by(order, by)
            .paginate(&db, constants::SEARCH_HOUSES_PAGE_COUNT);
        // 查询页面, 从索引0开始
        let houses = pagenate.fetch_page(page - 1).await.unwrap();
        let mut house_info: Vec<SearchHouse> = Vec::new();
        for house in houses {
            let owner = IhUsers::find_by_id(house.user_id.unwrap())
                .one(&db)
                .await
                .unwrap()
                .unwrap();
            house_info.push(SearchHouse {
                house_id: house.id,
                title: house.title.unwrap(),
                img_url: house.default_image_url.unwrap(),
                owner_img_url: owner.image_url.unwrap_or("".to_string()),
                price: house.price.unwrap(),
                room_count: house.room_count.unwrap(),
                order_count: house.order_count.unwrap(),
                address: house.address.unwrap(),
            });
        }
        data = SearchHouseRes {
            house_info,
            current_page: page,
            total_page: pagenate.num_pages().await.unwrap(),
        };
        // 存入缓存
        redis_con
            .set_ex::<&String, String, i32>(
                redis_key,
                serde_json::to_string(&SearchCache { data: data.clone() }).unwrap(),
                constants::SEARCH_HOUSES_EXPIRE,
            )
            .await
            .unwrap();
    }
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data,
    }))
}

// 获取预定的房屋
pub async fn get_order_house(
    _auth_user: AuthUser,
    Path(house_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<BookingHouseRes>>, RetError> {
    let house = IhHouses::find_by_id(house_id).one(&db).await.unwrap();
    let house = house
        .ok_or(RetError::DATAERR("house_id不存在".to_string()))
        .unwrap();
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: BookingHouseRes {
            house_id,
            title: house.title.unwrap(),
            img_url: house.default_image_url.unwrap_or("".to_string()),
            price: house.price.unwrap(),
        },
    }))
}
