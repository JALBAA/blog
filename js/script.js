(function(a){var t=1e3;var e=a("#totop");var o=500;a(window).scroll(function(){var o=a(document).scrollTop();if(o>t){a(e).stop().fadeTo(300,1)}else{a(e).stop().fadeTo(300,0)}});a(e).click(function(){a("html, body").animate({scrollTop:0},o);return false});var r=a("#search-form-wrap"),n=false,i=200;var s=function(){n=true};var l=function(a){setTimeout(function(){n=false;a&&a()},i)};a("#nav-search-btn").on("click",function(){if(n)return;s();r.addClass("on");l(function(){a(".search-form-input").focus()})});a(".search-form-input").on("blur",function(){s();r.removeClass("on");l()});a("body").on("click",function(){a(".article-share-box.on").removeClass("on")}).on("click",".article-share-link",function(t){t.stopPropagation();var e=a(this),o=e.attr("data-share"),r=e.offset();if(o=="baidu"){var n=a("#article-share-box");shareDataUrl=e.attr("data-url");shareDataTitle=e.attr("data-title");if(n.hasClass("on")){n.removeClass("on");return}a(".article-share-box.on").hide();n.css({top:r.top+25,left:r.left-25}).addClass("on")}else{var i=e.attr("data-url"),s=encodeURIComponent(i),l="article-share-box-"+e.attr("data-id");if(a("#"+l).length){var n=a("#"+l);if(n.hasClass("on")){n.removeClass("on");return}}else{var c=['<div id="'+l+'" class="article-share-box">','<input class="article-share-input" value="'+i+'">','<div class="article-share-links">','<a href="https://twitter.com/intent/tweet?url='+s+'" class="article-share-twitter" target="_blank" title="Twitter"></a>','<a href="https://www.facebook.com/sharer.php?u='+s+'" class="article-share-facebook" target="_blank" title="Facebook"></a>','<a href="http://pinterest.com/pin/create/button/?url='+s+'" class="article-share-pinterest" target="_blank" title="Pinterest"></a>','<a href="https://plus.google.com/share?url='+s+'" class="article-share-google" target="_blank" title="Google+"></a>',"</div>","</div>"].join("");var n=a(c);a("body").append(n)}a(".article-share-box.on").hide();n.css({top:r.top+25,left:r.left}).addClass("on")}}).on("click",".article-share-box",function(a){a.stopPropagation()}).on("click",".article-share-box-input",function(){a(this).select()}).on("click",".article-share-box-link",function(a){a.preventDefault();a.stopPropagation();window.open(this.href,"article-share-box-window-"+Date.now(),"width=500,height=450")});a(".article-entry").each(function(t){a(this).find("img").each(function(){if(a(this).parent().hasClass("fancybox"))return;var t=this.alt;if(t)a(this).after('<span class="caption">'+t+"</span>");a(this).wrap('<a href="'+this.src+'" title="'+t+'" class="fancybox"></a>')});a(this).find(".fancybox").each(function(){a(this).attr("rel","article"+t)})});if(a.fancybox){a(".fancybox").fancybox()}var c=a("#container"),f=false,h=200;var u=function(){f=true};var p=function(){setTimeout(function(){f=false},h)};a("#main-nav-toggle").on("click",function(){if(f)return;u();c.toggleClass("mobile-nav-on");p()});a("#wrap").on("click",function(){if(f||!c.hasClass("mobile-nav-on"))return;c.removeClass("mobile-nav-on")})})(jQuery);