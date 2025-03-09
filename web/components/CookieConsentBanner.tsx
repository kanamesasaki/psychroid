"use client";

import { useState, useEffect } from "react";
import CookieConsent from "react-cookie-consent";

export default function CookieConsentBanner() {
    const [isClient, setIsClient] = useState(false);

    useEffect(() => {
        setIsClient(true);
    }, []);

    if (!isClient) return null;

    return (
        <CookieConsent
            location="bottom"
            buttonText="Accept"
            cookieName="ga-consent"
            style={{ background: "#2B373B" }}
            buttonStyle={{ background: "#4e503b", fontSize: "13px" }}
            expires={150}
        >
            This website uses cookies to enhance the user experience.{" "}
        </CookieConsent>
    );
}