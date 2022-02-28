function getCookie(name) {
    var r = document.cookie.match("\\b" + name + "=([^;]*)\\b");
    return r ? r[1] : undefined;
}

$(document).ready(function() {
    $("#mobile").focus(function(){
        $("#mobile-err").hide();
    });
    $("#password").focus(function(){
        $("#password-err").hide();
    });
    $(".form-login").submit(function(e){
        //阻止默认的表单提交行为
        e.preventDefault();
        mobile = $("#mobile").val();
        passwd = $("#password").val();
        if (!mobile) {
            $("#mobile-err span").html("请填写正确的手机号！");
            $("#mobile-err").show();
            return;
        } 
        if (!passwd) {
            $("#password-err span").html("请填写密码!");
            $("#password-err").show();
            return;
        }
        //发送ajax请求
        //js对象数据转化为json格式
        var postData = JSON.stringify({phone:mobile, password:passwd});
        $.ajax({
            url: 'api/v1.0/sessions',
            type: 'post',
            data: postData,
            contentType: 'application/json',
            headers: {'X-CSRFToken': getCookie('csrf_token')},
            dataType: 'json',
            success: function (resp) {
                if(resp.errno == '0'){
                    //登录成功，跳转到首页
                    location.href='/index.html';
                }else{
                    //登录失败
                    alert(resp.errmsg);
                }
            }
        })
    });
})