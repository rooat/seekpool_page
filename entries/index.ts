import "../css/styles.css";
// declare var jQuery: (selector: string) => any; 
(async () => {
  // Note: files in `crate/pkg/` will be created on the first build.
    await import("../seekpool/pkg/index");
    if(window.location.href != window.location.protocol+"//"+window.location.host+"/"){
        window.location.href="/";
    }
    // miner_pie()
})();


