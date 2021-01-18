var _____WB$wombat$assign$function_____ = function(name) {return (self._wb_wombat && self._wb_wombat.local_init && self._wb_wombat.local_init(name)) || self[name]; };
if (!self.__WB_pmw) { self.__WB_pmw = function(obj) { this.__WB_source = obj; return this; } }
{
  let window = _____WB$wombat$assign$function_____("window");
  let self = _____WB$wombat$assign$function_____("self");
  let document = _____WB$wombat$assign$function_____("document");
  let location = _____WB$wombat$assign$function_____("location");
  let top = _____WB$wombat$assign$function_____("top");
  let parent = _____WB$wombat$assign$function_____("parent");
  let frames = _____WB$wombat$assign$function_____("frames");
  let opener = _____WB$wombat$assign$function_____("opener");

function byid(id) {
    return document.getElementById(id);
}
function valof(id) {
    return byid(id).checked ? 1 : 0;
}
function text(e) {
    if (e == null) {
        return "";
    }
    var s = e.value;
    return s;
}
 
var lastQ = null
var ticking = false
var ticksSinceChange = 0
 
function update() {
    var q = theImage();
//    byid("xxx").innerHTML = "<img src=\"" + q + "\" />"
//    alert(q)
//    return;

    var req = new XMLHttpRequest()
    req.onreadystatechange = function() {
        if(req.readyState == 4 && req.status == 200) {
            byid("output").innerHTML = req.responseText
        //    if(history && history.replaceState)
        //        history.replaceState(null, null, "?q=" + encodeURIComponent(q))
        }
    }
    req.open("GET", q, true)
    req.send(null)
    
    _gaq.push(['_trackEvent', 'QR', 'Generate', q])
}

function theImage() {
    return "/qr/draw?i=" + img + "&u=" + encodeURI(byid("url").value) + "&m=" + m + "&x=" + dx + "&y=" + dy + "&v=" + v + "&c=" + valof("control") + "&r=" + valof("rand") + "&d=" + valof("data") + "&t=" + valof("dither") + "&s=" + Math.floor(Math.random()*1e9) + "&z=" + z + "&o=" + rotation
}
   
  var _gaq = _gaq || [];
  _gaq.push(['_setAccount', 'UA-3319603-5']);
  _gaq.push(['_trackPageview']);

  (function() {
    var ga = document.createElement('script'); ga.type = 'text/javascript'; ga.async = true;
    ga.src = ('https:' == document.location.protocol ? 'https://web.archive.org/web/20200523010551/https://ssl' : 'https://web.archive.org/web/20200523010551/http://www') + '.google-analytics.com/ga.js';
    var s = document.getElementsByTagName('script')[0]; s.parentNode.insertBefore(ga, s);
  })();

var dx=4
var dy=4
var url="https://web.archive.org/web/20200523010551/http://research.swtch.com/qart"
var m=2
var v=6
var z=0
var img="pjw"
var cheat=0
var rotation=0

function up() { dy++; update(); }
function down() { dy--; update(); }
function left() { dx++; update(); }
function right() { dx--; update(); }
function bigger() { if(v < 8) { v++; update(); } }
function smaller() { if(v > 1) { v--; update(); } }
function setimg(s) { img=s; update(); }
function togglemask() { if(m>=0) m=-1; else m=2; update(); }
function ibigger() { z++; update(); }
function ismaller() { z--; update(); }
function rotate() { rotation = (rotation+1) & 3; update(); }


}
/*
     FILE ARCHIVED ON 01:05:51 May 23, 2020 AND RETRIEVED FROM THE
     INTERNET ARCHIVE ON 04:43:45 Jan 18, 2021.
     JAVASCRIPT APPENDED BY WAYBACK MACHINE, COPYRIGHT INTERNET ARCHIVE.

     ALL OTHER CONTENT MAY ALSO BE PROTECTED BY COPYRIGHT (17 U.S.C.
     SECTION 108(a)(3)).
*/
/*
playback timings (ms):
  captures_list: 103.117
  exclusion.robots: 0.078
  exclusion.robots.policy: 0.07
  RedisCDXSource: 2.514
  esindex: 0.006
  LoadShardBlock: 84.791 (3)
  PetaboxLoader3.datanode: 212.434 (5)
  CDXLines.iter: 13.842 (3)
  load_resource: 396.469 (2)
  PetaboxLoader3.resolve: 147.345 (2)
*/
