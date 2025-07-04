"use client";

import { useSearchParams } from 'next/navigation';
import Link from 'next/link'

export default function TermsPage() {

    const searchParams = useSearchParams();
    const from = searchParams.get('from');

    const fromAuth = from === 'auth'

    return (
        <div className="container mx-auto my-10 px-4">
            {fromAuth && (
                <div className="mb-6">
                    <Link
                        href="/auth"
                        className="text-sm text-muted-foreground hover:text-primary"
                    >
                        ← Back to Auth
                    </Link>
                </div>
            )}

            <h1 className="text-3xl font-bold mb-6">Terms of Service</h1>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">1. Acceptance of Terms</h2>
                <p className="mb-4">
                    By accessing and using this service, you agree to comply with all terms and conditions.
                    If you do not agree, please do not use the service.
                </p>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">2. Service Description</h2>
                <p className="mb-4">
                    This service provides image search and management features.
                    We reserve the right to modify or discontinue the service without notice.
                </p>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">3. User Responsibilities</h2>
                <ul className="list-disc pl-6 mb-4 space-y-2">
                    <li>You must not use this service for any illegal activities</li>
                    <li>You must not upload or share content that infringes others' rights</li>
                    <li>You are responsible for protecting your account information</li>
                </ul>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">4. Privacy Policy</h2>
                <p className="mb-4">
                    Your privacy is important to us. Our Privacy Policy explains how we collect,
                    use and protect your personal information.
                </p>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">5. Disclaimer</h2>
                <p className="mb-4">
                    The service is provided "as is" without warranties of any kind.
                    We do not guarantee uninterrupted or error-free service.
                </p>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">6. Terms Modification</h2>
                <p className="mb-4">
                    We reserve the right to modify these terms. Updated terms will take effect
                    immediately upon posting. Continued use constitutes acceptance.
                </p>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">7. Contact Us</h2>
                <p className="mb-4">
                    If you have any questions about these Terms, please contact us at support@example.com.
                </p>
            </section>
        </div>
    )
}