function getCookie(name) {
    var r = document.cookie.match("\\b" + name + "=([^;]*)\\b");
    return r ? r[1] : undefined;
}

function generateUUID() {
    var d = new Date().getTime();
    if(window.performance && typeof window.performance.now === "function"){
        d += performance.now(); //use high-precision timer if available
    }
    var uuid = 'xxxxxx'.replace(/[xy]/g, function(c) {
        var r = (d + Math.random()*16)%16 | 0;
        d = Math.floor(d/16);
        return (c=='x' ? r : (r&0x3|0x8)).toString(16);
    });
    return uuid;
}

$(document).ready(function(){
    // $('.popup_con').fadeIn('fast');
    // $('.popup_con').fadeOut('fast');

    //默认设置房屋标题
    var title = generateUUID()
    $('#house-title').val(title);

    //获取地区信息
    $.get("api/v1.0/areas", function (resp) {
        if (resp.errno == '0'){
            //获取到城区数据
            var areas = resp.data;
            // 方法一:通过jQuery循环, 在select下面添加option标签
            // for (var key in areas){
            //     //<option value="1">东区</option>
            //     $("#area-id").append('<option value="'+key+'">' + areas[key] + '</option>')
            // }
            // 方法二:使用art-template前端模板
            // 给模板传值, 返回html文本
            var html = template('area-option', {areas: areas});
            // 设置select中的文本对象
            $('#area-id').html(html);
        }else {
            //存在错误
            alert(resp.errmsg);
        }
    }, 'json');

    //发布房源表单的提交
    $('#form-house-info').submit(function (e) {
        //阻止默认
        e.preventDefault();
        //自定义提交
        //获取数据
        var data = {};
        //获取form表单中的所有input提交项, 使用map循环每个input获取name和value
        $(this).serializeArray().map(function (item) {
            data[item.name] = item.value;
        })
        //获取复选框中勾选的设施项
        var facility_ids = [];
        $('input:checkbox:checked').each(function (index, item) {
            facility_ids[index] = $(item).val();
        })
        data["facilities"] = facility_ids;
        //转为json
        var data = JSON.stringify(data);

        //根据new_house_id判断是更新还是创建
        var newHouseID = $('#new-house-id').val();
        if (newHouseID){
            var type = 'PUT';
        }
        else{
            var type = 'POST';
        }
        $.ajax({
            url: 'api/v1.0/houses',
            type: type,
            contentType: 'application/json',
            data: data,
            headers: {'X-CSRFToken': getCookie('csrf_token')},
            dataType: 'json',
            success: function (resp) {
                if (resp.errno == '0'){
                    //发布成功
                    //设置house_id
                    $('#house-id').val(resp.data.house_id);
                    $('#new-house-id').val(resp.data.house_id);
                    //修改按钮提示
                    $('.btn-commit').val('修改房源信息');
                    //展示上传图片表单
                    $('#form-house-image').show();
                    alert('保存成功, 请上传房屋图片');
                }else {
                    //发布失败
                    alert(resp.errmsg)
                }
            }
        })
    })

    //上传房屋图片的表单
    $('#form-house-image').submit(function (e) {
        //阻止表单默认提交行为
        e.preventDefault()
        //自定义提交行为
        $(this).ajaxSubmit({
            url: 'api/v1.0/houses/images',
            type: 'POST',
            headers: {'X-CSRFToken': getCookie('csrf_token')},
            dataType: 'json',
            success: function (resp) {
                if (resp.errno == '0'){
                    //上传成功, 展示图片
                    $('.house-image-cons').append('<img src="' + resp.data.url + '"></img>')
                }else {
                    //上传失败
                    alert(resp.errmsg)
                }
            }
        })
    })
})