import { generateFeed } from '@/utils/feed'

export async function GET() {
  try {
    const feed = await generateFeed({ lang: 'en' })
    let rssXml = feed.rss2()
    
    // Add XML stylesheet
    rssXml = rssXml.replace(
      '<?xml version="1.0" encoding="utf-8"?>',
      '<?xml version="1.0" encoding="utf-8"?>\n<?xml-stylesheet href="/feeds/rss-style.xsl" type="text/xsl"?>',
    )

    return new Response(rssXml, {
      headers: {
        'Content-Type': 'application/rss+xml; charset=utf-8',
      },
    })
  } catch (error) {
    console.error('Error generating English RSS:', error)
    return new Response('Error generating RSS feed', { status: 500 })
  }
}

export const prerender = true 