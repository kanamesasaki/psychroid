import { MetadataRoute } from 'next'

// Add these export statements at the top
export const dynamic = 'force-static'
export const revalidate = false // or a number for revalidation period in seconds

export default function sitemap(): MetadataRoute.Sitemap {
    const baseUrl = process.env.NEXT_PUBLIC_BASE_URL || 'https://psychroid.thermocraft.space/'

    return [
        {
            url: `${baseUrl}/`,
            lastModified: new Date(),
            changeFrequency: 'weekly',
            priority: 1.0,
        },
        {
            url: `${baseUrl}/contact`,
            lastModified: new Date(),
            changeFrequency: 'monthly',
            priority: 0.5,
        },
        // Add more pages as the application grows...
    ]
}