// Inside vue.config.js
module.exports = {
    // ...other vue-cli plugin options...
    pwa: {
      name: "Clup",
      themeColor: '#1761a0',
      msTileColor: '#1761a0',
      appleMobileWebAppCapable: 'yes',
      appleMobileWebAppStatusBarStyle: 'black',
  
      // configure the workbox plugin
      workboxPluginMode: 'GenerateSW',
      //workboxOptions: {
        // swSrc is required in InjectManifest mode.
       // swSrc: 'dev/sw.js',
        // ...other Workbox options...
      //}
    
  }
}