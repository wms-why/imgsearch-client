'use client'

import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { checkApiKey, saveUserInfo } from "@/data/auth";
import { Link } from "lucide-react";
import { useState } from "react";


export default function App() {

  const [apiKey, setApiKey] = useState("");
  const [validError, setValidError] = useState<string | null>(null);

  async function save() {
    let r = await checkApiKey(apiKey);

    setValidError(null);
    if (typeof r === 'string') {
      setValidError("check apikey failed: " + r);
      return false;
    } else {
      saveUserInfo(r);
      return true;
    }
  }

  return <>
    <Card className="w-full max-w-md">
      <CardHeader>
        <CardTitle className="text-2xl">Step 1: API Key Setup</CardTitle>
        <CardDescription>
          Please enter your API key to continue
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <Input
          placeholder="Enter your API Key"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
        />
        <p className="text-sm text-muted-foreground">
          Don't have an API key?{' '}
          <Link
            href="https://imgsearch.dev/blog/how-to-get-api-key?ref=imgsearch-client"
            target="_blank"
            className="text-primary hover:underline"
          >
            Get one here
          </Link>
        </p>

        <Button className="mb-6" variant="ghost" onClick={save}>Save</Button>
        {validError && <p className="text-sm text-red-500">{validError}</p>}
      </CardContent>
    </Card></>
}