"use client";

import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card";
import { useToast } from "@/components/ui/use-toast";
import { get, GuideType, next } from "@/data/guide";
import Link from "next/link";
import ImgdirPage from "../imgdir/page";
import { set } from "date-fns";
import { boolean } from "zod";
import { checkApiKey, saveUserInfo } from "@/data/auth";

interface StepInfo {
  order: number;
  type: GuideType;
  title: string;
}

const steps: StepInfo[] = [
  {
    order: 1,
    type: "auth",
    title: "Add your apikey",
  },
  {
    order: 2,
    type: "imgdir",
    title: "Add your image directory",
  },
  {
    order: 3,
    type: "finished",
    title: "All done!",
  },
];
export default function GuidePage() {
  const [step, setStep] = useState<GuideType>("auth");
  const [stepInfo, setStepInfo] = useState<StepInfo>();
  const [apiKey, setApiKey] = useState("");
  const [validError, setValidError] = useState<string | null>();
  const { toast } = useToast();

  useEffect(() => {
    const loadStep = async () => {
      let currentStep = await get();
      if (!currentStep) {
        currentStep = await next();
      }
      setStep(currentStep);
    };
    // loadStep();
  }, []);

  useEffect(() => {
    const s = steps.filter(e => e.type === step)[0];
    setStepInfo(s);
  }, [step]);

  const stepValid = async () => {
    setValidError(null);
    if (step === "auth") {
      const r = await checkApiKey(apiKey);

      if (typeof r === 'string') {
        setValidError("check apikey failed: " + r);
        return false;
      } else {
        saveUserInfo(r);
        return true;
      }
    }

    return false;

  }

  const handleNext = async () => {
    try {
      if (await stepValid()) {
        const nextStep = await next();
        setStep(nextStep);
      };

    } catch (error) {
      toast({
        title: "Error",
        description: error instanceof Error ? error.message : "Failed to proceed",
        variant: "destructive",
      });
    }
  };

  return (
    <div className="container flex flex-col items-center justify-center gap-6 py-10 md:py-20">
      <h1 className="font-bold text-3xl">{stepInfo?.order}.Guide - {stepInfo?.title}</h1>

      {step === 'auth' && (
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
          </CardContent>
        </Card>
      )}

      {step === 'imgdir' && (
        <ImgdirPage />
      )}

      {step === 'finished' && (
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle className="text-2xl">Setup Complete</CardTitle>
            <CardDescription>
              You have completed all setup steps
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Button asChild className="w-full">
              <Link href="/search">Start Searching</Link>
            </Button>
          </CardContent>
        </Card>

      )}


      {validError && (
        <div className="text-red-500"> {validError}</div>
      )}

      {step != 'finished' &&
        <Button
          onClick={handleNext}
          className="w-1/2"
        >
          Continue
        </Button>}


    </div>
  );
}