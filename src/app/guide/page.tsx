"use client";

import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card";
import { useToast } from "@/components/ui/use-toast";
import { get, GuideType, next } from "@/data/guide";
import Link from "next/link";
import ImgdirPage from "../imgdir/page";
import TokenPage from "../token/page";
import { getApikey } from "@/data/auth";
import { getAll } from "@/data/img-dirs";

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
    loadStep();
  }, []);

  useEffect(() => {
    const s = steps.filter(e => e.type === step)[0];
    setStepInfo(s);
  }, [step]);

  const stepValid = async () => {
    setValidError(null);
    if (step === "auth") {
      let apikey = await getApikey();

      if (apikey === undefined) {
        setValidError("apikey is empty");
        return false;
      }

      return true;
    }

    if (step === "imgdir") {
      const r = await getAll();

      const valid = r.length != 0;

      if (!valid) {
        setValidError("image dirs is empty");
        return false;
      }
      return true;
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
      {/* <h1 className="font-bold text-3xl">{stepInfo?.order}.Guide - {stepInfo?.title}</h1> */}

      <Card className="w-full md:2/3">
        <CardHeader>
          <CardTitle className="text-2xl  text-center">{stepInfo?.order}.Guide - {stepInfo?.title}</CardTitle>
          <CardDescription className="text-center">
            Please finish the guide to proceed
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {step === 'auth' && (
            <TokenPage />
          )}

          {step === 'imgdir' && (
            <ImgdirPage />
          )}

          {step === 'finished' && (
            <Button asChild className="w-full">
              <Link href="/search">Start Searching</Link>
            </Button>
          )}
        </CardContent>
      </Card>




      {/* {step === 'finished' && (
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle className="text-2xl">Setup Complete</CardTitle>
            <CardDescription>
              You have completed all setup steps
            </CardDescription>
          </CardHeader>
          <CardContent>
            
          </CardContent>
        </Card>

      )} */}


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