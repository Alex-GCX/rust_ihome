function hrefBack() {
    history.go(-1);
}

function getCookie(name) {
    var r = document.cookie.match("\\b" + name + "=([^;]*)\\b");
    return r ? r[1] : undefined;
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
    //获取url的数据, 原封不动直接传给后端进行校验
    var url_param = document.location.search.substr(1)
    //发送ajax请求, 修改订单状态
    $.ajax({
        url: 'api/v1.0/orders/alipay',
        type: 'PATCH',
        data: url_param,
        headers: {'X-CSRFToken': getCookie('csrf_token')},
        dataType: 'json',
        success: function (resp) {
            if (resp.errno == '0'){
                //计时跳转到我的订单
                function jump(count) {
                    window.setTimeout(function () {
                        count--;
                        if (count > 0) {
                            $('#num').text(count);
                            jump(count);
                        } else {
                            location.href = "orders.html";
                        }
                    }, 1000);
                }
                jump(3);
            }else {
                alert(resp.errmsg);
            }
        }
    });
})
