function hrefBack() {
    history.go(-1);
}

function decodeQuery(){
    var search = decodeURI(document.location.search);
    return search.replace(/(^\?)/, '').split('&').reduce(function(result, item){
        values = item.split('=');
        result[values[0]] = values[1];
        return result;
    }, {});
}

$(document).ready(function(){
    // 获取url的参数, id和f
    var url_param = decodeQuery()
    // 发送ajax请求获取房屋数据
    if (url_param !== undefined) {
        $.get('api/v1.0/houses/' + url_param.id, function (resp) {
            if (resp.errno == '0') {
                //获取成功
                var data = resp.data;
                //使用template设置图片
                $('.swiper-container').html(template('house-images', {houseImages: data.house.img_urls, price: data.house.price}))
                //展示其他信息
                $('.detail-con').html(template('house-info', {house: data.house}))
                //不是当前用户不是房东则展示即刻预定按钮
                if (data.user_id != data.house.owner_id) {
                    $(".book-house").show();
                }

                var mySwiper = new Swiper ('.swiper-container', {
                    loop: true,
                    autoplay: 2000,
                    autoplayDisableOnInteraction: false,
                    pagination: '.swiper-pagination',
                    paginationType: 'fraction'
                });

                //设置预定按钮的跳转url
                if (data.user_id != -1){
                    //登录了则进入预定界面
                    var url = '/booking.html?id=' + url_param.id;
                }else{
                    //未登录则进入登录界面
                    var url = '/login.html';
                }
                $('.book-house').attr('href', url)
            } else if (resp.errno == "4003") {
                location.href='/login.html';
            } else {
                //获取失败
                alert(resp.errmsg)
            }
        }, 'json');
    };
})