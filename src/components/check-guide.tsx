'use client';

import { useEffect } from "react";
import { isFinished } from "@/data/guide";
import { useRouter } from "next/navigation"; // 注意：app目录要用 next/navigation

export default function CheckGuide() {
  const router = useRouter();

  useEffect(() => {
    isFinished().then(finished => {
      if (!finished) {
        router.push("/guide");
      }
    });
  }, [router]);

  return null; // 不渲染任何东西，只做副作用
}
