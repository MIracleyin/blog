// .vuepress/config.js
module.exports = {
    title: 'Hello VuePress',
    description: 'Just playing around',
    themeConfig: {
        logo: '/assets/img/logo.png',
        ///////////////////////////////////////////
        nav: [
            { text: '根', link: '/' },
            { text: '序言', link: '/Preface/' },
            { text: '光影', 
              link: '/Picture/',
              items: [
                  {text: '2020', link: '/Picture/2020'},
                  {text: '2019', link: '/Picture/2019'}, 
              ]
            },
            { text: '留痕', 
              link: '/Review/',
              items: [
                {text: '影评', link: '/Review/film'},
                {text: '文评', link: '/Review/read'},
              ]
            },
            { text: '技术', link: '/technology/' },
            { text: 'External', link: 'https://google.com' },
        ],
        ///////////////////////////////////////////
        sidebar: [
            '/',
            '/page-a',
            ['/page-b', 'Explicit link text']
        ]
    }
}