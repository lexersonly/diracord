"use client";

import { useEffect, useRef } from "react";

/**
 * The 3D swirl behind the hero: nested iridescent rings turning slowly and
 * leaning toward the pointer. three.js is loaded only in the browser, after
 * the page is interactive. With "reduce motion" on, a single frame is drawn.
 */
export function HeroScene({ className }: { className?: string }) {
  const host = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const el = host.current;
    if (!el) return;
    let disposed = false;
    let cleanup = () => {};

    void import("three").then((THREE) => {
      if (disposed) return;
      const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
      let renderer: InstanceType<typeof THREE.WebGLRenderer>;
      try {
        renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true, powerPreference: "low-power" });
      } catch {
        return; // no WebGL: the CSS gradient behind stays
      }
      renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
      renderer.toneMapping = THREE.ACESFilmicToneMapping;
      renderer.toneMappingExposure = 1.15;
      el.appendChild(renderer.domElement);
      renderer.domElement.style.cssText = "width:100%;height:100%;display:block";

      const scene = new THREE.Scene();
      const camera = new THREE.PerspectiveCamera(32, 1, 0.1, 100);
      camera.position.set(0, 0, 11);

      const group = new THREE.Group();
      scene.add(group);
      const rings: InstanceType<typeof THREE.Mesh>[] = [];
      const COUNT = 7;
      for (let i = 0; i < COUNT; i++) {
        const t = i / (COUNT - 1);
        const geometry = new THREE.TorusGeometry(0.45 + i * 0.4, 0.13 + t * 0.14, 64, 220);
        const material = new THREE.MeshPhysicalMaterial({
          color: new THREE.Color().setHSL(0.72 - t * 0.1, 0.75, 0.16 + t * 0.05),
          metalness: 0.85,
          roughness: 0.32,
          clearcoat: 1,
          clearcoatRoughness: 0.18,
          iridescence: 1,
          iridescenceIOR: 1.7,
          iridescenceThicknessRange: [180, 620],
        });
        const ring = new THREE.Mesh(geometry, material);
        ring.rotation.set(0.9 + i * 0.13, 0.2 + i * 0.22, i * 0.5);
        ring.scale.z = 1.8; // flatten the tube into a ribbon
        group.add(ring);
        rings.push(ring);
      }

      const lights: [number, number, [number, number, number]][] = [
        [0xa78bfa, 900, [8, 6, 9]],
        [0x3b82f6, 800, [-9, -4, 8]],
        [0xe879f9, 500, [5, -8, 6]],
        [0x2dd4bf, 300, [-6, 8, 5]],
      ];
      for (const [color, intensity, pos] of lights) {
        const light = new THREE.PointLight(color, intensity, 60);
        light.position.set(...pos);
        scene.add(light);
      }
      scene.add(new THREE.AmbientLight(0x3b2a8c, 2.2));

      const pointer = { x: 0, y: 0 };
      const onMove = (e: PointerEvent) => {
        pointer.x = (e.clientX / window.innerWidth) * 2 - 1;
        pointer.y = (e.clientY / window.innerHeight) * 2 - 1;
      };
      window.addEventListener("pointermove", onMove, { passive: true });

      const resize = () => {
        const { clientWidth: w, clientHeight: h } = el;
        if (!w || !h) return;
        renderer.setSize(w, h, false);
        camera.aspect = w / h;
        camera.updateProjectionMatrix();
        // On wide screens the swirl sits to the right of the text.
        group.position.x = w > 900 ? 2.5 : 0;
        group.position.y = w > 900 ? 0.1 : 1.4;
        group.scale.setScalar(w > 900 ? 1 : 0.72);
        renderer.render(scene, camera);
      };
      const observer = new ResizeObserver(resize);
      observer.observe(el);

      let frame = 0;
      let visible = true;
      const io = new IntersectionObserver(([entry]) => {
        visible = entry.isIntersecting;
      });
      io.observe(el);
      const clock = new THREE.Clock();
      const tick = () => {
        frame = requestAnimationFrame(tick);
        if (!visible) return;
        const t = clock.getElapsedTime();
        rings.forEach((ring, i) => {
          const dir = i % 2 ? -1 : 1;
          ring.rotation.z = i * 0.5 + t * dir * (0.1 + i * 0.012);
          ring.rotation.x = 0.9 + i * 0.13 + Math.sin(t * 0.35 + i) * 0.12;
        });
        group.rotation.y += (pointer.x * 0.3 - group.rotation.y) * 0.04;
        group.rotation.x += (pointer.y * 0.2 - group.rotation.x) * 0.04;
        renderer.render(scene, camera);
      };
      resize();
      if (!reduce) tick();

      cleanup = () => {
        cancelAnimationFrame(frame);
        observer.disconnect();
        io.disconnect();
        window.removeEventListener("pointermove", onMove);
        rings.forEach((r) => {
          r.geometry.dispose();
          (r.material as InstanceType<typeof THREE.Material>).dispose();
        });
        renderer.dispose();
        renderer.domElement.remove();
      };
    });

    return () => {
      disposed = true;
      cleanup();
    };
  }, []);

  return <div ref={host} className={className} aria-hidden />;
}
