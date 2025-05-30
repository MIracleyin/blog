import { generateFeed } from '@/utils/feed'

export async function GET() {
  try {
    const feed = await generateFeed({ lang: 'en' })
    let atomXml = feed.atom1()
    
    // Add XML stylesheet
    atomXml = atomXml.replace(
      '<?xml version="1.0" encoding="utf-8"?>',
      '<?xml version="1.0" encoding="utf-8"?>\n<?xml-stylesheet href="/feeds/atom-style.xsl" type="text/xsl"?>',
    )

    return new Response(atomXml, {
      headers: {
        'Content-Type': 'application/atom+xml; charset=utf-8',
      },
    })
  } catch (error) {
    console.error('Error generating English Atom:', error)
    return new Response('Error generating Atom feed', { status: 500 })
  }
}

export const prerender = true 