function showSuccessMsg() {
    $('.popup_con').fadeIn('fast', function() {
        setTimeout(function(){
            $('.popup_con').fadeOut('fast',function(){});
        },1000)
    });
}

function getCookie(name) {
    var r = document.cookie.match("\\b" + name + "=([^;]*)\\b");
    return r ? r[1] : undefined;
}

$(document).ready(function () {
    //获取当前头像
    $.get('api/v1.0/users/info', function (resp) {
        if (resp.errno == '0'){
            //获取成功
            $('#user-avatar').attr("src", resp.data.url);
            $('#user-name').attr("value", resp.data.name);
        }else if (resp.errno == '4101'){
            //未登录
            location.href = 'login.html'
        }
    }, 'json')
    //页面一加载，阻止form表单默认的提交行为
    $('#form-avatar').submit(function (e) {
        e.preventDefault();
        //执行自定义的行为
        //利用jquery.min.js提供的ajaxSubmit插件对表单进行异步提交
        $(this).ajaxSubmit({
            url: '/api/v1.0/users/images',
            type: 'patch',
            dataType: 'json',
            headers: {'X-CSRFToken': getCookie('csrf_token')},
            success: function (resp) {
                if (resp.errno == '0'){
                    //上传成功
                    $('#user-avatar').attr('src', resp.data.url);
                }else if (resp.errno == '4101'){
                    //未登录
                    location.href = 'login.html'
                }else {
                    //上传失败
                    alert('上传失败:'+resp.errmsg);
                }
            }
        })
    });
    //页面一加载，阻止表单默认行为
    $('#form-name').submit(function (e) {
        //阻止默认提交
        e.preventDefault()
        //设置表单提交
        //获取表单数据
        var name = $('#user-name').val()
        if (!name){
            alert('用户名必填')
        }
        data = JSON.stringify({name: name})
        $.ajax({
            url: 'api/v1.0/users/names',
            type: 'patch',
            contentType: 'application/json',
            data: data,
            dataType: 'json',
            headers: {'X-CSRFToken': getCookie('csrf_token')},
            success: function (resp) {
                if (resp.errno == '0') {
                    //设置成功
                    location.href='my.html'
                }else if (resp.errno == '4101'){
                    //未登录
                    location.href = 'login.html'
                }else {
                    //设置失败
                    alert(resp.errmsg)
                }
            }
        })
    })
})