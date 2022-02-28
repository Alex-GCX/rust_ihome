function showSuccessMsg() {
    $('.popup_con').fadeIn('fast', function() {
        setTimeout(function(){
            $('.popup_con').fadeOut('fast',function(){});
            location.href = 'auth.html';
        },1000) 
    });
}

function getCookie(name){
    var ret = document.cookie.match('\\b' + name + '=([^;]*)\\b');
    return ret ? ret[1] : undefined;
}

//页面一加载
$(document).ready(function (){
    //发送请求获取已有的认证信息
    $.get('/api/v1.0/users/auth', function (resp) {
        if (resp.errno == '0'){
            //获取成功
            var realName = resp.data.real_name;
            var idCard = resp.data.real_id_card;
            if (realName && idCard){
                //设置展示
                $('#real-name').attr({'value': realName, 'readonly': 'readonly'});
                $('#id-card').attr({'value': idCard, 'readonly': 'readonly'});
                $('.btn-success').hide();
            }
        }else if (resp.errno == '4101'){
            //未登录
            location.href = 'login.html'
        }
    }, 'json')
    //设置表单提交事件
    $('#form-auth').submit(function (e) {
        //禁止默认表单提交事件
        e.preventDefault()
        //设置表单提交事件
        //获取表单数据
        var realName = $('#real-name').val()
        if (!realName){
            alert('真实姓名必填')
            return;
        }
        var idCard = $('#id-card').val()
        if (!idCard){
            alert('身份证号码必填')
            return;
        }
        data = JSON.stringify({real_name: realName, real_id_card: idCard})
        //发送请求
        $.ajax({
            url: 'api/v1.0/users/auth',
            type: 'patch',
            data: data,
            contentType: 'application/json',
            dataType: 'json',
            headers: {'X-CSRFToken': getCookie('csrf_token')},
            success: function (resp) {
                if (resp.errno == '0'){
                    //认证成功
                    showSuccessMsg();
                }else{
                    //认证失败
                    alert(resp.errmsg);
                }
            }
        })
    })
})