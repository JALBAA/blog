---
title: 微信Ios下音视频自动播放
date: 2016-02-18 23:16:22
author: jalbaa
keywords: 微信,audio,ios,浏览器,自动播放
toc: true
tag:
    - tips
---
 查过很多资料，关于ios下的资料，苹果官方是禁止自动播放音视频的，只能通过用户点击事件触发播放音视频。

 比如stackoverflow上的这个[讨论](http://stackoverflow.com/questions/3009888/autoplay-audio-files-on-an-ipad-with-html5)

 今天偶然查到，微信上，**WeixinJSBridgeReady** 事件的回调函数中，调用audio(video)的play方法，可以实现自动播放，看来微信是对safari做了特殊处理了。

 话说，WeiXinJSBridge按理说，16年起应该已经改用wxjssdk了，但是今天试了试还是能用，感觉略神奇，至少ready还能用。。

代码如下

 ```javascript
//new 一个音频对象
var audio = document.createElement('audio')
audio.src = "//xxx.xxx.mp3"
body.append(aduio)
//侦听weixinjsbridge的ready事件
document.addEventListener('WeixinJSBridgeReady',function(){
    //这样就能播放了
    audio.play();
})
 ```
 如果是微信jssdk用户
 直接在wx.ready回调函数内调用，更加保险，毕竟是官方支持的

```javascript
wx.ready(function(){
    //这样就能播放了
    audio.play();
})
```


 ps:　safari还是不能自动播放的啊。。所以最好判断下微信ua，分别对待

 ```javascript
//如果不是微信浏览器
//就老老实实的用点击的方式播放把
if(!navigator.userAgent.match(/MicroMessenger/i)){
    audio.addEventListener('click',function(){
        auido.play();
    })
}
 ```
