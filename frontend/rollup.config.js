import svelte from "rollup-plugin-svelte";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import livereload from "rollup-plugin-livereload";
import terser from "@rollup/plugin-terser";
import autoPreprocess from "svelte-preprocess";
import typescript from "@rollup/plugin-typescript";
import eslint from "@rollup/plugin-eslint";
import postcss from 'rollup-plugin-postcss';


const production = !process.env.ROLLUP_WATCH;

export default {
  input: "src/main.js",
  output: {
    sourcemap: true,
    format: "iife",
    name: "app",
    file: "public/bundle.js",
  },
  plugins: [
    svelte({
      // enable run-time checks when not in production
      dev: !production,
      preprocess: autoPreprocess(),

      // we'll extract any component CSS out into
      // a separate file  better for performance
      css: (css) => {
        css.write("bundle.css");
      },
    }),
    postcss({
      extensions: ['.css'],
    }),
    typescript({ sourceMap: true }),


    // If you have external dependencies installed from
    // npm, you'll most likely need these plugins. In
    // some cases you'll need additional configuration 
    // consult the documentation for details:
    // https://github.com/rollup/rollup-plugin-commonjs
    resolve(),
    commonjs(),
    eslint({
      fix: true,
      // include: ["src/**"], // used if we want auto fix all the time
    }),

    // Watch the `public` directory and refresh the
    // browser on changes when not in production
    !production && livereload("public"),

    // If we're building for production (npm run build
    // instead of npm run dev), minify
    production && terser(),
  ],
  watch: {
    clearScreen: false,
  },
};
