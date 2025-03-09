import ContactForm from "@/components/ContactForm";

export const metadata = {
    title: "Contact | Psychrometric Chart Calculator",
    description: "Get in touch with us for questions about the psychrometric chart tool.",
};

export default function ContactPage() {
    return (
        <div className="container mx-auto px-4 py-8">
            <ContactForm />
        </div>
    );
}