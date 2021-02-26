$(document).ready(function(){
    $(".header-burger").click (function(event) {
        $(".header-burger,.main-nav").toggleClass("active");
        $("body").toggleClass("lock");

    });

});