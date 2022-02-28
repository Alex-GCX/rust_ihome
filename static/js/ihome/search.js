//初始化页面全局变量
var curr_page = 1;
var next_page = 1;
var total_page = 1;
var house_data_querying = true;   //表示正在查询过程中, 则此时不能再发送查询请求

//解析url
function decodeQuery(){
    var search = decodeURI(document.location.search);
    return search.replace(/(^\?)/, '').split('&').reduce(function(result, item){
        values = item.split('=');
        result[values[0]] = values[1];
        return result;
    }, {});
}

//更新日期
function updateFilterDateDisplay() {
    var startDate = $("#start-date").val();
    var endDate = $("#end-date").val();
    var $filterDateTitle = $(".filter-title-bar>.filter-title").eq(0).children("span").eq(0);
    if (startDate) {
        var text = startDate.substr(5) + "/" + endDate.substr(5);
        $filterDateTitle.html(text);
    } else {
        $filterDateTitle.html("入住日期");
    }
}

//定义发送搜索请求的方法, 参数action为表示追加显示查询结果还是刷新显示查询结果
function send_ajax(action){
    //从html中获取查询条件
    var areaId = $('.filter-area li[class="active"]').attr('area-id');
    var startDate = $("#start-date").val();
    var endDate = $("#end-date").val();
    var sortedBy = $(".filter-sort li[class='active']").attr('sort-key');
    //处理查询条件
    if (areaId == undefined){
        areaId = ''
    }
    if (startDate == undefined){
        startDate = ''
    }
    if (endDate == undefined){
        endDate = ''
    }
    if (sortedBy == undefined){
        sortedBy = 'new'
    }
    //append追加则查询下一页, 否则重新查询第一页
    if (action == 'append'){
        page = next_page
    }else {
        page = 1
    }
    //发送ajax请求执行查询
    var searchUrl ='api/v1.0/search/houses?aid='+areaId+'&sd='+startDate+'&ed='+endDate+'&page='+page+'&sorted_by='+sortedBy;
    $.get(searchUrl, function (resp) {
        //进入回调函数, 则把查询状态改为false
        house_data_querying = false;
        if (resp.errno == '0'){
            //查询成功
            total_page = resp.data.total_page;
            //使用模板设置查询结果
            if (action == 'append'){
                //拼接展示这一页的信息
                curr_page = page
                $('.house-list').append(template('search-houses', {houses: resp.data.house_info}));
            }else{
                //重置当前页为1
                curr_page = 1
                next_page = 1
                //重新查询覆盖
                $('.house-list').html(template('search-houses', {houses: resp.data.house_info}));
            }
        }else{
            //查询失败
            alert(resp.errmsg);
        }
    }, 'json');
}


$(document).ready(function(){
    //获取url查询条件
    var queryData = decodeQuery();
    //提取查询条件
    var areaName = queryData["aname"];
    var startDate = queryData["sd"];
    var endDate = queryData["ed"];
    var areaId = queryData["aid"]
    //将url的查询日期设置到日期查询框中
    $("#start-date").val(startDate); 
    $("#end-date").val(endDate); 
    updateFilterDateDisplay();
    //url中不存在地区条件地区选择框显示'位置区域'
    if (!areaName) areaName = "位置区域";
    $(".filter-title-bar>.filter-title").eq(1).children("span").eq(0).html(areaName);

    //发送ajax请求查询地区信息
    $.get('api/v1.0/areas', function (resp) {
        if (resp.errno == '0'){
            //获取成功, 添加地区列表html, 将url中的地区ID的li标签添加active属性
            for (var i=1; i<Object.keys(resp.data).length+1; i++){
                if (parseInt(areaId) == i){
                    $(".filter-area").append('<li area-id="' + i + '" class="active">' + resp.data[i] + '</li>')
                }else{
                    $(".filter-area").append('<li area-id="' + i + '">' + resp.data[i] + '</li>')
                }
            }
            // 发送查询请求
            send_ajax('refresh');
        }
    }, 'json');

    //这一步会在回调函数之前执行, 所以获取不到值, 需要放到上面的回调函数中
    // send_ajax('refresh');

    // 获取页面显示窗口的高度
    var windowHeight = $(window).height();
    // 为窗口的滚动添加事件函数
    window.onscroll=function(){
        // var a = document.documentElement.scrollTop==0? document.body.clientHeight : document.documentElement.clientHeight;
        var b = document.documentElement.scrollTop==0? document.body.scrollTop : document.documentElement.scrollTop;
        var c = document.documentElement.scrollTop==0? document.body.scrollHeight : document.documentElement.scrollHeight;
        // 如果滚动到接近窗口底部
        if(c-b<windowHeight+50){
            // 如果没有正在向后端发送查询房屋列表信息的请求
            if (!house_data_querying) {
                // 将正在向后端查询房屋列表信息的标志设置为真，
                house_data_querying = true;
                // 如果当前页面数还没到达总页数
                if(curr_page < total_page) {
                    // 将要查询的页数设置为当前页数加1
                    next_page = curr_page + 1;
                    // 向后端发送请求，查询下一页房屋数据
                    send_ajax('append');
                } else {
                    house_data_querying = false;
                }
            }
        }
    }

    $(".input-daterange").datepicker({
        format: "yyyy-mm-dd",
        startDate: "today",
        language: "zh-CN",
        autoclose: true
    });

    //统一管理查询条件选择框
    var $filterItem = $(".filter-item-bar>.filter-item");
    $(".filter-title-bar").on("click", ".filter-title", function(e){
        var index = $(this).index();
        if (!$filterItem.eq(index).hasClass("active")) {
            $(this).children("span").children("i").removeClass("fa-angle-down").addClass("fa-angle-up");
            $(this).siblings(".filter-title").children("span").children("i").removeClass("fa-angle-up").addClass("fa-angle-down");
            $filterItem.eq(index).addClass("active").siblings(".filter-item").removeClass("active");
            $(".display-mask").show();
        } else {
            $(this).children("span").children("i").removeClass("fa-angle-up").addClass("fa-angle-down");
            $filterItem.eq(index).removeClass('active');
            $(".display-mask").hide();
            updateFilterDateDisplay();
        }
    });

    //地区选择框的点击事件
    $(".filter-item-bar>.filter-area").on("click", "li", function(e) {
        if (!$(this).hasClass("active")) {
            $(this).addClass("active");
            $(this).siblings("li").removeClass("active");
            $(".filter-title-bar>.filter-title").eq(1).children("span").eq(0).html($(this).html());
        } else {
            $(this).removeClass("active");
            $(".filter-title-bar>.filter-title").eq(1).children("span").eq(0).html("位置区域");
        }
        //点击后隐藏选择框
        $('.filter-area').removeClass("active");
        $(".display-mask").click();
    });

    //排序选择框的点击事件
    $(".filter-item-bar>.filter-sort").on("click", "li", function(e) {
        if (!$(this).hasClass("active")) {
            $(this).addClass("active");
            $(this).siblings("li").removeClass("active");
            $(".filter-title-bar>.filter-title").eq(2).children("span").eq(0).html($(this).html());
            //点击后隐藏选择框
            $('.filter-sort').removeClass("active");
            $(".display-mask").click();
        }
    })

    //查询条件底部灰框的点击事件
    $(".display-mask").on("click", function(e) {
        $(this).hide();
        $filterItem.removeClass('active');
        updateFilterDateDisplay();
        // 执行查询
        send_ajax('refresh');
    });

})