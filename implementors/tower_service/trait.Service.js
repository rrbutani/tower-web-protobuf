(function() {var implementors = {};
implementors["tower_service"] = [];
implementors["tower_web"] = [{text:"impl&lt;S&gt; <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a> for <a class=\"struct\" href=\"tower_web/middleware/cors/struct.CorsService.html\" title=\"struct tower_web::middleware::cors::CorsService\">CorsService</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"tower_web/util/http/trait.HttpService.html\" title=\"trait tower_web::util::http::HttpService\">HttpService</a>,&nbsp;</span>",synthetic:false,types:["tower_web::middleware::cors::service::CorsService"]},{text:"impl&lt;S, RequestBody, ResponseBody&gt; <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a> for <a class=\"struct\" href=\"tower_web/middleware/deflate/struct.DeflateService.html\" title=\"struct tower_web::middleware::deflate::DeflateService\">DeflateService</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a>&lt;Request = <a class=\"struct\" href=\"http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;RequestBody&gt;, Response = <a class=\"struct\" href=\"http/response/struct.Response.html\" title=\"struct http::response::Response\">Response</a>&lt;ResponseBody&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ResponseBody: <a class=\"trait\" href=\"tower_web/util/buf_stream/trait.BufStream.html\" title=\"trait tower_web::util::buf_stream::BufStream\">BufStream</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"tower_service/trait.Service.html#associatedtype.Error\" title=\"type tower_service::Service::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a>,&nbsp;</span>",synthetic:false,types:["tower_web::middleware::deflate::service::DeflateService"]},{text:"impl&lt;S, RequestBody, ResponseBody&gt; <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a> for <a class=\"struct\" href=\"tower_web/middleware/log/struct.LogService.html\" title=\"struct tower_web::middleware::log::LogService\">LogService</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a>&lt;Request = <a class=\"struct\" href=\"http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;RequestBody&gt;, Response = <a class=\"struct\" href=\"http/response/struct.Response.html\" title=\"struct http::response::Response\">Response</a>&lt;ResponseBody&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"tower_service/trait.Service.html#associatedtype.Error\" title=\"type tower_service::Service::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a>,&nbsp;</span>",synthetic:false,types:["tower_web::middleware::log::service::LogService"]},{text:"impl&lt;T, U&gt; <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a> for <a class=\"struct\" href=\"tower_web/routing/struct.RoutedService.html\" title=\"struct tower_web::routing::RoutedService\">RoutedService</a>&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tower_web/routing/trait.Resource.html\" title=\"trait tower_web::routing::Resource\">Resource</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: <a class=\"trait\" href=\"tower_web/error/trait.Catch.html\" title=\"trait tower_web::error::Catch\">Catch</a>,&nbsp;</span>",synthetic:false,types:["tower_web::routing::service::RoutedService"]},{text:"impl&lt;T, U, M&gt; <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a> for <a class=\"struct\" href=\"tower_web/service/struct.WebService.html\" title=\"struct tower_web::service::WebService\">WebService</a>&lt;T, U, M&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tower_web/routing/trait.Resource.html\" title=\"trait tower_web::routing::Resource\">Resource</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: <a class=\"trait\" href=\"tower_web/error/trait.Catch.html\" title=\"trait tower_web::error::Catch\">Catch</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;M: <a class=\"trait\" href=\"tower_web/util/http/trait.HttpMiddleware.html\" title=\"trait tower_web::util::http::HttpMiddleware\">HttpMiddleware</a>&lt;<a class=\"struct\" href=\"tower_web/routing/struct.RoutedService.html\" title=\"struct tower_web::routing::RoutedService\">RoutedService</a>&lt;T, U&gt;&gt;,&nbsp;</span>",synthetic:false,types:["tower_web::service::web::WebService"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a> for <a class=\"struct\" href=\"tower_web/util/http/struct.LiftService.html\" title=\"struct tower_web::util::http::LiftService\">LiftService</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tower_web/util/http/trait.HttpService.html\" title=\"trait tower_web::util::http::HttpService\">HttpService</a>,&nbsp;</span>",synthetic:false,types:["tower_web::util::http::service::LiftService"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
