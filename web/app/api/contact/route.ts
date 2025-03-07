import { NextResponse } from 'next/server';
import nodemailer from 'nodemailer';

export async function POST(request: Request) {
    try {
        const body = await request.json();
        const { name, email, message } = body;

        // Validate required fields
        if (!name || !email || !message) {
            return NextResponse.json(
                { error: 'Name, email, and message are required' },
                { status: 400 }
            );
        }

        // Configure simpler mail transport
        const transporter = nodemailer.createTransport({
            host: 'smtp.gmail.com',
            port: 587,
            secure: false, // Use TLS
            auth: {
                user: process.env.EMAIL_USER, // Email address
                pass: process.env.EMAIL_PASS  // App password
            }
        });

        // Email content
        const mailOptions = {
            from: `"Psychroid Website" <${process.env.EMAIL_USER}>`,
            to: 'support@thermocraft.space',
            replyTo: email,
            subject: `[Psychroid Contact] Message from ${name}`,
            text: `
Name: ${name}
Email: ${email}

Message:
${message}
            `,
            html: `
                <h3>New contact form submission from Psychroid website</h3>
                <p><strong>From:</strong> ${name} (${email})</p>
                <p><strong>Message:</strong></p>
                <p>${message.replace(/\n/g, '<br>')}</p>
            `
        };

        // Send email
        await transporter.sendMail(mailOptions);

        return NextResponse.json({ success: true });
    } catch (error) {
        console.error('Error sending email:', error);
        return NextResponse.json(
            { error: 'Failed to send message. Please try again or contact us directly.' },
            { status: 500 }
        );
    }
}