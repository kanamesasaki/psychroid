"use client";

import Link from "next/link";
import { Github } from "lucide-react"; // Icon

const Header = () => {
    return (
        <header className="border-b px-4 py-1 mb-3 flex items-center justify-between">
            <div>
                <Link href="/" className="text-2xl font-bold">
                    Psychroid: Psychrometric Chart Tool
                </Link>
            </div>
            <nav className="flex items-center space-x-5">
                <Link href="/contact" className="text-sm font-medium hover:text-primary">
                    Contact
                </Link>
                <a
                    href="https://github.com/kanamesasaki/psychroid"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-sm font-medium hover:text-primary flex items-center"
                >
                    <Github className="w-4 h-4 mr-1" /> GitHub
                </a>
                {/* Language selection dropdown */}
                {/* <select defaultValue="en" className="border-gray-300 rounded px-2 py-1 text-sm">
                    <option value="en">EN</option>
                    <option value="ja">JA</option>
                </select> */}
            </nav>
        </header>
    );
};

export default Header;