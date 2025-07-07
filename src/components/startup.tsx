'use client';

import { useEffect } from "react";
import { onStartup as imgDirsStartup } from "@/data/img-dirs";

export default function Startup() {
  useEffect(() => {
    imgDirsStartup();
  }, []);
  return null; // 不渲染任何东西，只做副作用
}
