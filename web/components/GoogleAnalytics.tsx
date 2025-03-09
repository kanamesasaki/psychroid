"use client";

import Script from "next/script";

export const GoogleAnalytics = () => {
    return (
        <>
            <Script
                strategy="afterInteractive"
                src="https://www.googletagmanager.com/gtag/js?id=G-NR7TMX1P1S"
            />
            <Script
                id="google-analytics"
                strategy="afterInteractive"
                dangerouslySetInnerHTML={{
                    __html: `
            window.dataLayer = window.dataLayer || [];
            function gtag(){dataLayer.push(arguments);}
            gtag('js', new Date());
            gtag('config', 'G-NR7TMX1P1S');
          `,
                }}
            />
        </>
    );
};

export default GoogleAnalytics;