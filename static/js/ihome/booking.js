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

function showErrorMsg() {
    $('.popup_con').fadeIn('fast', function() {
        setTimeout(function(){
            $('.popup_con').fadeOut('fast',function(){}); 
        },1000) 
    });
}

$(document).ready(function(){
    //
    $(".input-daterange").datepicker({
        format: "yyyy-mm-dd",
        startDate: "today",
        language: "zh-CN",
        autoclose: true
    });
    //
    $(".input-daterange").on("changeDate", function(){
        var startDate = $("#start-date").val();
        var endDate = $("#end-date").val();

        if (startDate && endDate && startDate > endDate) {
            showErrorMsg();
        } else {
            var sd = new Date(startDate);
            var ed = new Date(endDate);
            days = (ed - sd)/(1000*3600*24);
            var price = $(".house-text>p>span").html();
            var amount = days * parseFloat(price);
            $(".order-amount>span").html(amount.toFixed(2) + "(共"+ days +"晚)");
        }
    });

    //获取url中的房屋ID
    var houseId = decodeQuery()['id'];
    //发送ajax请求, 获取房屋数据
    $.get('api/v1.0/booking/houses/'+houseId, function (resp) {
        if (resp.errno == '0'){
            //展示房屋信息
            $('.house-info img').attr('src', resp.data.img_url);
            $('.house-text h3').html(resp.data.title);
            $('.house-text span').html(resp.data.price);
        }else {
            alert(resp.errmsg);
        }
    }, 'json')

    //设置表单自定义提交
    $('.submit-btn').click(function (e) {
        //阻止默认的form表单提交
        e.preventDefault();
        //自定义提交
        var startDate = $('#start-date').val();
        var endDate = $('#end-date').val();
        var data = JSON.stringify({house_id: houseId, start_date: startDate, end_date: endDate});
        //提交ajax请求
        $.ajax({
            url: 'api/v1.0/orders',
            type: 'POST',
            data: data,
            contentType: 'application/json',
            headers: {'X-CSRFToken': getCookie('csrf_token')},
            dataType: 'json',
            success: function (resp) {
                if (resp.errno == '0'){
                    //创建成功
                    location.href='/orders.html';
                }else {
                    alert(resp.errmsg);
                }
            }
        })
    });
})
