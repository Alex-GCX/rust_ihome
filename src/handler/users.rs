use crate::entity::ih_users;
use crate::entity::prelude::IhUsers;
use crate::utils::{
    jwt::{jwt_encode, AuthUser},
    response_codes::{ResponseInfo, RetError},
};
use axum::{
    extract::{Extension, Multipart},
    http::header::{HeaderMap, HeaderValue, SET_COOKIE},
    Json,
};
use chrono::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{entity::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

// -------------request-------------------

#[derive(Debug, Deserialize)]
pub struct RegisterReq {
    mobile: String,
    password: String,
    password2: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    phone: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct SetUserNameReq {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct SetUserAuthReq {
    real_name: String,
    real_id_card: String,
}

// -------------response-------------------

#[derive(Debug, Serialize)]
pub struct UserRes {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoRes {
    name: String,
    url: String,
    mobile: String,
}

#[derive(Debug, Serialize)]
pub struct UserImageRes {
    url: String,
}

#[derive(Debug, Serialize)]
pub struct UserAuthRes {
    real_name: String,
    real_id_card: String,
}

// -------------handler-------------------

// 注册
pub async fn register(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<RegisterReq>,
) -> Result<(HeaderMap, Json<ResponseInfo<String>>), RetError> {
    let phone = payload.mobile;
    let password = payload.password;
    let password2 = payload.password2;
    // 判断是否为空
    if phone.is_empty() || password.is_empty() || password2.is_empty() {
        return Err(RetError::PARAMERR(
            "字段'phone','password','password2'不能为空".to_string(),
        ));
    }
    // 判断两个密码是否相同
    if password != password2 {
        return Err(RetError::PARAMERR("两次密码不一致".to_string()));
    }
    // 判断是否注册过
    let user = IhUsers::find()
        .filter(ih_users::Column::Phone.eq(phone.clone()))
        .one(&db)
        .await
        .unwrap();
    if user != None {
        return Err(RetError::DATAERR("手机号已注册过".to_string()));
    }
    // 创建用户
    // let now = NaiveDateTime::from_timestamp(Local::now().timestamp(), 0);
    let user = ih_users::ActiveModel {
        name: Set(Some(phone.clone())),
        phone: Set(Some(phone)),
        password_hash: Set(Some(password)),
        ..Default::default()
    };
    let res = IhUsers::insert(user).exec(&db).await.unwrap();
    // 设置cookie
    set_cookie(res.last_insert_id)
}

// 获取用户信息
pub async fn get_user(
    auth_user: AuthUser,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<UserRes>>, RetError> {
    let user = IhUsers::find_by_id(auth_user.user_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: UserRes {
            name: user.name.unwrap(),
        },
    }))
}

// 登录
pub async fn login(
    Json(payload): Json<LoginReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<(HeaderMap, Json<ResponseInfo<String>>), RetError> {
    let phone = payload.phone;
    let password = payload.password;
    // 校验
    let query = IhUsers::find()
        .filter(ih_users::Column::Phone.eq(phone.clone()))
        .filter(ih_users::Column::PasswordHash.eq(password.clone()))
        .one(&db)
        .await
        .unwrap();
    let user = query.ok_or(RetError::PARAMERR("手机号或密码不正确".to_string()))?;
    // 设置cookie
    set_cookie(user.id)
}

// 登出
pub async fn logout(_user: AuthUser) -> Result<Json<ResponseInfo<String>>, RetError> {
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: "".to_string(),
    }))
}

// 设置cookie
fn set_cookie(user_id: i32) -> Result<(HeaderMap, Json<ResponseInfo<String>>), RetError> {
    // 设置cookie
    let user = AuthUser {
        user_id,
        exp: 10000000000,
    };
    // jwt加密生成token
    let token = jwt_encode(user)
        .map_err(|_| RetError::TOKENERR("生成token异常".to_string()))
        .unwrap();
    // 设置cookie头
    let mut headers = HeaderMap::new();
    // 将token放在cookie头中
    let jwt_token = HeaderValue::from_str(&("jwt_token=".to_string() + &token)).unwrap();
    headers.insert(SET_COOKIE, jwt_token);
    // 返回header和json
    Ok((
        headers,
        Json(ResponseInfo {
            errno: "0".to_string(),
            data: "".to_string(),
        }),
    ))
}

// userinfo-个人信息页
pub async fn get_user_info(
    auth_user: AuthUser,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<UserInfoRes>>, RetError> {
    let user = IhUsers::find_by_id(auth_user.user_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    let use_info = ResponseInfo {
        errno: "0".to_string(),
        data: UserInfoRes {
            name: user.name.unwrap(),
            url: user.image_url.unwrap_or("".to_string()),
            mobile: user.phone.unwrap(),
        },
    };
    Ok(Json(use_info))
}

// 头像上传
pub async fn set_user_image(
    auth_user: AuthUser,
    Extension(db): Extension<DatabaseConnection>,
    mut multipart: Multipart,
) -> Result<Json<ResponseInfo<UserImageRes>>, RetError> {
    // 获取时间戳
    let timestamp = Local::now().timestamp().to_string();
    let url = format!("static/images/users/{}.jpg", timestamp);
    // 读取传入的文件
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        if name != "avatar".to_string() {
            continue;
        }
        // 保存文件
        let mut file = File::create(&url).unwrap();
        file.write(&data).unwrap();
    }
    if url != "".to_string() {
        // 更新数据库
        IhUsers::update_many()
            .col_expr(ih_users::Column::ImageUrl, Expr::value(url.clone()))
            .filter(ih_users::Column::Id.eq(auth_user.user_id))
            .exec(&db)
            .await
            .unwrap();
    }
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: UserImageRes { url },
    }))
}

// 设置用户名
pub async fn set_user_name(
    auth_user: AuthUser,
    Json(payload): Json<SetUserNameReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<String>>, RetError> {
    let name = payload.name;
    // 查询用户名是否已存在
    let user = IhUsers::find()
        .filter(ih_users::Column::Name.eq(name.clone()))
        .one(&db)
        .await
        .unwrap();
    if user != None {
        return Err(RetError::DATAERR("用户名已存在".to_string()));
    }
    // 更新数据库
    IhUsers::update_many()
        .col_expr(ih_users::Column::Name, Expr::value(name))
        .filter(ih_users::Column::Id.eq(auth_user.user_id))
        .exec(&db)
        .await
        .unwrap();
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: "".to_string(),
    }))
}

// 实名认证接口
pub async fn get_user_auth(
    auth_user: AuthUser,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<UserAuthRes>>, RetError> {
    let user = IhUsers::find_by_id(auth_user.user_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let user_auth = UserAuthRes {
        real_name: user.real_name.unwrap_or("".to_string()),
        real_id_card: user.real_id_card.unwrap_or("".to_string()),
    };

    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: user_auth,
    }))
}

// 更新实名认证信息
pub async fn authenticate(
    auth_user: AuthUser,
    Json(payload): Json<SetUserAuthReq>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseInfo<String>>, RetError> {
    let real_name = payload.real_name;
    let real_id_card = payload.real_id_card;
    // 查询该用户是否已经实名认证过
    let user = IhUsers::find_by_id(auth_user.user_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    if user.real_name != None && user.real_id_card != None {
        return Err(RetError::DATAERR("用户以实名认证过".to_string()));
    }
    // 查询身份信息是否已被其他人实名认证
    let user = IhUsers::find()
        .filter(ih_users::Column::RealName.eq(real_name.clone()))
        .filter(ih_users::Column::RealIdCard.eq(real_id_card.clone()))
        .one(&db)
        .await
        .unwrap();
    if user != None {
        return Err(RetError::DATAERR("该身份已被认证过".to_string()));
    }
    // 更新数据库
    IhUsers::update_many()
        .col_expr(ih_users::Column::RealName, Expr::value(real_name))
        .col_expr(ih_users::Column::RealIdCard, Expr::value(real_id_card))
        .filter(ih_users::Column::Id.eq(auth_user.user_id))
        .exec(&db)
        .await
        .unwrap();
    Ok(Json(ResponseInfo {
        errno: "0".to_string(),
        data: "".to_string(),
    }))
}
