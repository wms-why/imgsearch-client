'use client';

import { useEffect } from "react";
import { onStartup as imgDirsStartup } from "@/data/img-dirs";

let startInit = false;
export default function Startup() {
  useEffect(() => {
    if (!startInit) {
      imgDirsStartup();
      startInit = true
    }
  }, []);
  return null; // 不渲染任何东西，只做副作用
}
