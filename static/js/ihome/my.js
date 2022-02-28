function getCookies(name) {
    var ret = document.cookie.match('\\b' + name + '=([^;]*)\\b');
    return ret ? ret[1] : undefined;
}

function logout() {
    $.ajax({
        url: 'api/v1.0/sessions',
        type: 'delete',
        dataType: 'json',
        headers: {
            'X-CSRFToken': getCookies('csrf_token')
        },
        success: function (resp) {
            if (resp.errno == '0'){
                location.href='/'
            }else{
                alert(resp.errmsg)
            }
        }
    })
}

$(document).ready(function(){
    //调用获取头像接口
    $.get('/api/v1.0/users/info', function (resp) {
        if (resp.errno == '0'){
            //获取成功，设置image的url
            $('#user-avatar').attr('src', resp.data.url);
            $('#user-name').html(resp.data.name);
            $('#user-mobile').html(resp.data.mobile);
        }else if (resp.errno == '4101'){
            //未登录
            location.href = 'login.html'
        }
    })
})