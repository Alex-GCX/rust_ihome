use crate::entity::prelude::{IhAreas, IhFacilities};
use crate::entity::{ih_areas, ih_facilities};
use axum::extract::Extension;
use sea_orm::{entity::Set, DatabaseConnection, EntityTrait};

pub async fn insert_areas(Extension(db): Extension<DatabaseConnection>) -> &'static str {
    let mut v: Vec<ih_areas::ActiveModel> = Vec::new();
    let areas = [
        "黄浦区".to_owned(),
        "徐汇区".to_owned(),
        "普陀区".to_owned(),
        "长宁区".to_owned(),
        "杨浦区".to_owned(),
        "静安区".to_owned(),
        "虹口区".to_owned(),
    ];
    for (id, area) in areas.iter().enumerate() {
        let a = ih_areas::ActiveModel {
            id: Set((id + 1).try_into().unwrap()),
            name: Set(Some(area.to_string())),
            ..Default::default()
        };
        v.push(a);
    }
    // let area1 = ih_areas::Entity::insert(area1).exec(&db).await.unwrap();
    IhAreas::insert_many(v).exec(&db).await.unwrap();
    "ok"
}

pub async fn insert_fac(Extension(db): Extension<DatabaseConnection>) -> &'static str {
    let mut v: Vec<ih_facilities::ActiveModel> = Vec::new();
    let facs = [
        "无线网络",
        "热水淋浴",
        "空调",
        "暖气",
        "允许吸烟",
        "饮水设备",
        "牙具",
        "香皂",
        "拖鞋",
        "手纸",
        "毛巾",
        "沐浴露、洗发露",
        "冰箱",
        "洗衣机",
        "电梯",
        "允许做饭",
        "允许带宠物",
        "允许聚会",
        "门禁系统",
        "停车位",
        "有线网络",
        "电视",
        "浴缸",
    ];
    for (id, fac) in facs.iter().enumerate() {
        let f = ih_facilities::ActiveModel {
            id: Set((id + 1).try_into().unwrap()),
            name: Set(Some(fac.to_string())),
            ..Default::default()
        };
        v.push(f);
    }
    IhFacilities::insert_many(v).exec(&db).await.unwrap();
    "ok"
}
