'use client'

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { checkApiKey, saveUserInfo } from "@/data/auth";
import { useState } from "react";
import { useToast } from "@/components/ui/use-toast";
import Link from "next/link";

export default function App() {
  const [apiKey, setApiKey] = useState("");
  const { toast } = useToast();

  async function save() {
    try {
      const r = await checkApiKey(apiKey);
      if (typeof r === 'string') {
        throw new Error(r);
      }
      saveUserInfo(r);
      toast({
        title: "Success",
        description: "API Key saved successfully",
      });
      return true;
    } catch (error) {
      toast({
        title: "Error",
        description: error instanceof Error ? error.message : "Failed to save API Key",
        variant: "destructive",
      });
      return false;
    }
  }

  return (
    <div className="container mx-auto py-8 space-y-4 text-center">
      <div className="space-y-2">
        <Input
          placeholder="Enter your API Key"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
        />
        <Link
          href="https://imgsearch.dev/blog/how-to-get-api-key?ref=imgsearch-client"
          target="_blank"
          className="text-primary hover:underline my-16"
        >
          Don't have an API key?
        </Link>
      </div>

      <Button onClick={save}>Save API Key</Button>
    </div>
  );
}