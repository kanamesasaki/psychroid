import type { NextApiRequest, NextApiResponse } from 'next';
import nodemailer from 'nodemailer';

const handler = async (req: NextApiRequest, res: NextApiResponse) => {
    if (req.method !== 'POST') {
        res.setHeader('Allow', ['POST']);
        return res.status(405).end(`Method ${req.method} Not Allowed`);
    }

    const { name, email, message } = req.body;
    if (!name || !email || !message) {
        return res.status(400).json({ error: 'Missing required fields' });
    }

    // Nodemailer のトランスポーター設定 (Gmail SMTP)
    let transporter = nodemailer.createTransport({
        service: 'gmail',
        auth: {
            user: process.env.GMAIL_USER,      // Google Workspace のメールアドレス
            pass: process.env.GMAIL_PASS,      // アプリパスワードなど
        },
    });

    try {
        await transporter.sendMail({
            from: process.env.GMAIL_USER,
            to: process.env.CONTACT_EMAIL,     // 送信先のアドレス
            subject: `New contact from ${name}`,
            text: `You have a new message from ${name} (${email}):\n\n${message}`,
        });

        return res.status(200).json({ message: 'Email sent successfully' });
    } catch (error) {
        console.error(error);
        return res.status(500).json({ error: 'Error sending email' });
    }
};

export default handler;