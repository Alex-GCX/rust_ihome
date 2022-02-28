//模态框居中的控制
function centerModals(){
    $('.modal').each(function(i){   //遍历每一个模态框
        var $clone = $(this).clone().css('display', 'block').appendTo('body');    
        var top = Math.round(($clone.height() - $clone.find('.modal-content').height()) / 2);
        top = top > 0 ? top : 0;
        $clone.remove();
        $(this).find('.modal-content').css("margin-top", top-30);  //修正原先已经有的30个像素
    });
}

function getCookie(name) {
    var r = document.cookie.match("\\b" + name + "=([^;]*)\\b");
    return r ? r[1] : undefined;
}

$(document).ready(function(){
    $('.modal').on('show.bs.modal', centerModals);      //当模态框出现的时候
    $(window).on('resize', centerModals);

    //发送ajax请求获取订单
    $.get('api/v1.0/orders', function (resp) {
        if (resp.errno == '0'){
            //填充页面内容
            $('.orders-list').html(template('orders-list-tmpl', {orders:resp.data}));

            //房屋图片点击事件
            $('img').click(function () {
                var house_id = $(this).attr('house-id');
                location.href = "detail.html?id="+house_id;
            });

            //取消按钮
            $(".order-cancel").on("click", function(){
                var orderId = $(this).parents("li").attr("order-id");
                $(".modal-cancel").attr("order-id", orderId);
                console.log(orderId)
            });

            //确定取消按钮
            $('.modal-cancel').on("click", function () {
                var orderId = $(this).attr("order-id");
                $.ajax({
                    url: '/api/v1.0/orders/cancel/'+orderId,
                    type: 'PATCH',
                    contentType: 'application/json',
                    headers: {'X-CSRFToken': getCookie('csrf_token')},
                    dataType: 'json',
                    success: function (resp) {
                        if (resp.errno == '0'){
                            //更新成功, 刷新页面
                            location.reload();
                        }else {
                            alert(resp.errmsg)
                        }
                    }
                });
            });

            //去支付按钮
            $('.order-pay').on("click", function () {
                var orderId = $(this).parents("li").attr("order-id");
                //发送ajax请求获取支付页面url
                $.ajax({
                    url: '/api/v1.0/orders/alipay',
                    type: 'POST',
                    contentType: 'application/json',
                    data: JSON.stringify({order_id: orderId}),
                    headers: {'X-CSRFToken': getCookie('csrf_token')},
                    dataType: 'json',
                    success: function (resp) {
                        if (resp.errno == '0'){
                            //成功, 新窗口打开支付宝链接
                            // location.href = resp.data.url;
                            location.reload();
                        }else {
                            alert(resp.errmsg);
                        }
                    }
                })
            });

            //评论按钮
            $(".order-comment").on("click", function(){
                var orderId = $(this).parents("li").attr("order-id");
                $(".modal-comment").attr("order-id", orderId);
            });
            //去评论按钮
            $('.modal-comment').on('click', function () {
                var orderId = $(this).attr('order-id');
                var comment = $('#comment').val()
                $.ajax({
                    url: '/api/v1.0/orders/comment/'+orderId,
                    type: 'PATCH',
                    data: JSON.stringify({comment: comment}),
                    contentType: 'application/json',
                    headers: {'X-CSRFToken': getCookie('csrf_token')},
                    dataType: 'json',
                    success: function (resp) {
                        if (resp.errno == '0'){
                            //更新成功, 刷新页面
                            location.reload();
                        }else {
                            alert(resp.errmsg)
                        }
                    }
                });
            });
        }else {
            alert(resp.errmsg);
        }
    })
});