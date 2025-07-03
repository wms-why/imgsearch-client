'use client'
import { useEffect, useState } from "react";
import GuidePage from "./guide/page";
import SearchPage from "./search/page"
import { get as getGuide, GuideType } from "@/data/guide";
import { init as initPath } from "@/data/file-tools";

export default function App() {

  const [guide, setGuide] = useState<GuideType>();

  useEffect(() => {

    initPath().then(() => {
      return getGuide();
    }).then(guide => setGuide(guide));
  })

  return guide === 'finished' ? <SearchPage /> : <GuidePage />;
}