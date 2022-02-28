// 城区缓存信息, 七天， 单位秒
pub const AREA_REDIS_EXPIRES: usize = 7 * 24 * 360;

// 评论展示条数
pub const COMMENT_DISPLAY_COUNTS: u64 = 10;

// 房屋详情缓存信息, 七天， 单位秒
pub const HOUSE_REDIS_EXPIRES: usize = 7 * 24 * 3600;

// 首页最多展示房屋数量
pub const INDEX_HOUSES_COUNT: i32 = 5;
pub const INDEX_HOUSES_EXPIRES: usize = 24 * 3600;

// 查询列表页每页数据量
pub const SEARCH_HOUSES_PAGE_COUNT: usize = 3;

// 查询列表页缓存信息
pub const SEARCH_HOUSES_EXPIRE: usize = 7 * 24 * 3600;

#[derive(Debug)]
pub enum OrderStatus {
    WaitAccept,
    WaitPayment,
    WaitComment,
    Completed,
    Cancelled,
    Rejected,
    Unknow,
}

impl OrderStatus {
    pub fn new(status: &str) -> Self {
        match status {
            "WAIT_ACCEPT" => OrderStatus::WaitAccept,
            "WAIT_PAYMENT" => OrderStatus::WaitPayment,
            "WAIT_COMMENT" => OrderStatus::WaitComment,
            "COMPLETED" => OrderStatus::Completed,
            "CANCELLED" => OrderStatus::Cancelled,
            "REJECTED" => OrderStatus::Rejected,
            _ => OrderStatus::Unknow,
        }
    }
    pub fn get_desc(&self) -> &str {
        match self {
            OrderStatus::WaitAccept => "待接单",
            OrderStatus::WaitPayment => "待支付",
            OrderStatus::WaitComment => "待评价",
            OrderStatus::Completed => "已完成",
            OrderStatus::Cancelled => "已取消",
            OrderStatus::Rejected => "已拒绝",
            _ => "未知",
        }
    }
    pub fn get_info(&self) -> &str {
        match self {
            OrderStatus::WaitAccept => "WAIT_ACCEPT",
            OrderStatus::WaitPayment => "WAIT_PAYMENT",
            OrderStatus::WaitComment => "WAIT_COMMENT",
            OrderStatus::Completed => "COMPLETED",
            OrderStatus::Cancelled => "CANCELLED",
            OrderStatus::Rejected => "REJECTED",
            _ => "Unknow",
        }
    }
}
