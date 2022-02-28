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

function setStartDate() {
    var startDate = $("#start-date-input").val();
    if (startDate) {
        $(".search-btn").attr("start-date", startDate);
        $("#start-date-btn").html(startDate);
        $("#end-date").datepicker("destroy");
        $("#end-date-btn").html("离开日期");
        $("#end-date-input").val("");
        $(".search-btn").attr("end-date", "");
        $("#end-date").datepicker({
            language: "zh-CN",
            keyboardNavigation: false,
            startDate: startDate,
            format: "yyyy-mm-dd"
        });
        $("#end-date").on("changeDate", function() {
            $("#end-date-input").val(
                $(this).datepicker("getFormattedDate")
            );
        });
        $(".end-date").show();
    }
    $("#start-date-modal").modal("hide");
}

function setEndDate() {
    var endDate = $("#end-date-input").val();
    if (endDate) {
        $(".search-btn").attr("end-date", endDate);
        $("#end-date-btn").html(endDate);
    }
    $("#end-date-modal").modal("hide");
}

function goToSearchPage(th) {
    var url = "/search.html?";
    url += ("aid=" + $(th).attr("area-id"));
    url += "&";
    var areaName = $(th).attr("area-name");
    if (undefined == areaName) areaName="";
    url += ("aname=" + areaName);
    url += "&";
    url += ("sd=" + $(th).attr("start-date"));
    url += "&";
    url += ("ed=" + $(th).attr("end-date"));
    location.href = url;
}

$(document).ready(function(){
    //发送ajax获取登录信息
    $.get("/api/v1.0/sessions", function (resp) {
        if (resp.errno == '0'){
            //已登录，显示登录用户名
            $(".user-info>.user-name").html(resp.data.name);
            $(".user-info").show();
        }else{
            //未登录，显示登录注册框
            $(".top-bar>.register-login").show();
        }
    }, "json")

    //发送ajax请求， 获取房屋信息
    $.get('/api/v1.0/index/houses', function (resp) {
        if (resp.errno == '0'){
            //获取成功
            //设置页面图片
            $('.swiper-wrapper').html(template('index-houses', {houses: resp.data}));
            //轮播图
            var mySwiper = new Swiper ('.swiper-container', {
                loop: true,
                autoplay: 2000,
                autoplayDisableOnInteraction: false,
                pagination: '.swiper-pagination',
                paginationClickable: true
            });
        }else {
            //获取失败
            alert(resp.errmsg);
        }
    }, 'json');

    //发送ajax请求获取地区信息
    $.get('api/v1.0/areas', function (resp) {
        if (resp.errno == '0'){
            //获取成功
            //设置城区信息
            $('.area-list').html(template('index-areas', {areas: resp.data}));
            //设置城区的点击事件
            $('.area-list a').click(function (e) {
                //展示选择的城区
                $('#area-btn').html($(this).html());
                //给搜索按钮添加选择的地区属性, 因为搜索按钮点击后会从自身获取搜索条件
                $('.search-btn').attr('area-id', $(this).attr('area-id'));
                $('.search-btn').attr('area-name', $(this).html());
                //隐藏城区选择框
                $('#area-modal').modal("hide");
            });
        }else {
            //获取失败
            alert(resp.errmsg);
        }
    }, 'json');

    $('.modal').on('show.bs.modal', centerModals);      //当模态框出现的时候
    $(window).on('resize', centerModals);               //当窗口大小变化的时候
    $("#start-date").datepicker({
        language: "zh-CN",
        keyboardNavigation: false,
        startDate: "today",
        format: "yyyy-mm-dd"
    });
    $("#start-date").on("changeDate", function() {
        var date = $(this).datepicker("getFormattedDate");
        $("#start-date-input").val(date);
    });
})