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


function updateOrder(orderId, data){
    var data_json=JSON.stringify(data);
    $.ajax({
        url: 'api/v1.0/orders/accept/'+orderId,
        type: 'PATCH',
        contentType: 'application/json',
        data: data_json,
        headers: {'X-CSRFToken': getCookie('csrf_token')},
        dataType: 'json',
        success: function (resp) {
            if (resp.errno == '0'){
                //接收成功, 刷新页面
                location.reload();
            }else {
                alert(resp.errmsg);
            }
        }
    })
}


$(document).ready(function(){
    $('.modal').on('show.bs.modal', centerModals);      //当模态框出现的时候
    $(window).on('resize', centerModals);
    //发送ajax请求, 获取订单数据
    $.get('api/v1.0/orders?role=lorder', function (resp) {
        if (resp.errno == '0'){
            $('.orders-list').html(template('orders-info', {orders: resp.data}));

            //图片点击事件
            $('img').click(function () {
                var houseId = $(this).attr('house-id');
                location.href="detail.html?id="+houseId;
            })

            //接单按钮
            $(".order-accept").on("click", function(){
                var orderId = $(this).parents("li").attr("order-id");
                $(".modal-accept").attr("order-id", orderId);
            });

            //确定接单按钮
            $('.modal-accept').on("click", function () {
                var orderId = $(this).attr("order-id");
                //更新状态
                var data = {action: 'accept'};
                updateOrder(orderId, data);
            })

            //拒单按钮
            $(".order-reject").on("click", function(){
                var orderId = $(this).parents("li").attr("order-id");
                $(".modal-reject").attr("order-id", orderId);
            });

            //确定拒单按钮
            $('.modal-reject').on("click", function () {
                var orderId = $(this).attr("order-id");
                var reason = $('#reject-reason').val();
                //更新状态
                var data = {action: 'reject', comment: reason};
                updateOrder(orderId, data);
            })
        }else{
            alert(resp.errmsg);
        }
    })
});