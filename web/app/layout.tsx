import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import GoogleAnalytics from "@/components/GoogleAnalytics";
import CookieConsentBanner from "@/components/CookieConsentBanner";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Psychrometric Chart Calculator | Online HVAC Tool",
  description: "Online tool for psychrometric calculations. Calculate humidity, enthalpy, wet-bulb temperature and plot psychrometric processes in your browser.",
  keywords: "psychrometric chart, psychrometric calculator, humidity calculator, HVAC tool, wet bulb temperature, enthalpy calculation",
  openGraph: {
    title: "Psychrometric Chart Calculator",
    description: "Free web-based psychrometric calculator and chart plotting tool",
    type: "website",
    url: "https://psychroid.thermocraft.space",
  }
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <head>
        <GoogleAnalytics />
      </head>
      <body className={`${geistSans.variable} ${geistMono.variable} antialiased`}>
        {children}
        <CookieConsentBanner />
      </body>
    </html>
  );
}
