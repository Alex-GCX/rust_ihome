$(document).ready(function () {
    //发送ajax请求获取实名认证信息
    $.get('api/v1.0/users/auth', function (resp) {
        if(resp.errno == '0' && resp.data.real_name && resp.data.real_id_card){
            //已实名制
            $(".auth-warn").hide();
        }else {
            $(".auth-warn").show();
            $(".new-house").hide();
        }
    }, 'json');
    //发送ajax请求获取我的房屋信息
    $.get('api/v1.0/users/houses', function (resp) {
        if (resp.errno == '0'){
            //使用art-template模板发送房屋信息, 获取html文本
            var html = template('list-house-info', {houses:resp.data});
            //将html文本放到合适的位置
            $('.houses-list').append(html);
        }else {
            alert(resp.errmsg);
        }
    }, 'json');
})