import { Compare } from "@/components/landing/compare";
import { ErrorsShowcase } from "@/components/landing/errors";
import { Faq } from "@/components/landing/faq";
import { Features } from "@/components/landing/features";
import { Hero } from "@/components/landing/hero";
import { Start } from "@/components/landing/start";
import { Stats } from "@/components/landing/stats";
import { Strip } from "@/components/landing/strip";

export default function Home() {
  return (
    <>
      <Hero />
      <Strip />
      <Compare />
      <Features />
      <ErrorsShowcase />
      <Stats />
      <Start />
      <Faq />
    </>
  );
}
