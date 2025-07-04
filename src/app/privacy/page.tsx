"use client";
import Link from 'next/link'
import { useSearchParams } from 'next/navigation';

export default function PrivacyPage() {
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

            <h1 className="text-3xl font-bold mb-6">Privacy Policy</h1>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">1. Information We Collect</h2>
                <p className="mb-4">
                    We do not collect any personal data except:
                </p>
                <ul className="list-disc pl-6 mb-4 space-y-2">
                    <li>The image directory you select for processing</li>
                    <li>Files within that directory for analysis and indexing</li>
                </ul>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">2. How We Use Information</h2>
                <p className="mb-4">
                    We strictly use the collected data only for:
                </p>
                <ul className="list-disc pl-6 mb-4 space-y-2">
                    <li>Processing and analyzing selected images</li>
                    <li>Creating search indexes for your local use</li>
                    <li>Uploaded images will be deleted after indexing completes</li>
                </ul>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">3. Information Sharing</h2>
                <p className="mb-4">
                    We do not share any data with third parties except:
                </p>
                <ul className="list-disc pl-6 mb-4 space-y-2">
                    <li>Required image processing with large model service providers</li>
                    <li>When legally compelled to do so</li>
                </ul>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">4. Data Security</h2>
                <p className="mb-4">
                    We implement maximum security measures for your sensitive data (API keys, tokens), but:
                </p>
                <ul className="list-disc pl-6 mb-4 space-y-2">
                    <li>No system can guarantee 100% security</li>
                    <li>Image data security is not within our protection scope</li>
                </ul>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">5. Your Choices</h2>
                <p className="mb-4">
                    You may:
                </p>
                <ul className="list-disc pl-6 mb-4 space-y-2">
                    <li>Update your account information</li>
                    <li>Opt-out of promotional communications</li>
                    <li>Request deletion of your data</li>
                </ul>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">6. Changes to This Policy</h2>
                <p className="mb-4">
                    We may update this policy and will notify you of significant changes.
                </p>
            </section>

            <section className="mb-8">
                <h2 className="text-2xl font-semibold mb-4">7. Contact Us</h2>
                <p className="mb-4">
                    For privacy-related questions, contact us at privacy@example.com.
                </p>
            </section>
        </div>
    )
}