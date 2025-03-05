# First Home Care Center Website

A modern, responsive website for First Home Care Center built with Rust, Trunk, and Yew.

## Features

- 🚀 Fast, lightweight, and secure (built with Rust + WebAssembly)
- 📱 Fully responsive for all device sizes
- 🎨 Modern UI with smooth animations
- ♿ Accessible design
- 🔍 SEO friendly structure

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Trunk](https://trunkrs.dev/#install) - for bundling the WebAssembly app
- [wasm32-unknown-unknown target](https://rustwasm.github.io/docs/book/game-of-life/setup.html) - for compiling Rust to WebAssembly

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/siulynot/fhcpr.org.git
   cd fhcpr.org
   ```

2. Install WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install Trunk:
   ```bash
   cargo install trunk
   ```

## Development

To start the development server:

```bash
trunk serve --open
```

This will open a browser window with the site running at `http://localhost:8080`.

## Building for Production

To build the website for production:

```bash
trunk build --release
```

The production files will be in the `dist` directory. These can be deployed to any static website hosting service.

## Project Structure

```
first-home-care/
├── Cargo.toml                 # Rust package manifest
├── index.html                 # HTML entry point
├── src/                       # Rust source code
│   ├── main.rs                # Main entry point
│   ├── app.rs                 # Main app component
│   ├── components/            # UI components
│   └── pages/                 # Page components
└── static/                    # Static assets
    ├── images/                # Image files
    ├── styles.css             # CSS styles
    └── favicon.ico            # Favicon
```

## Customization

- Images: Replace the images in the `static/images/` directory with your own.
- Content: Update the text in the component files under `src/components/`.
- Styles: Modify the `static/styles.css` file to customize colors, fonts, etc.

## Improvements from the Original WordPress Template

1. **Modern Architecture:**
   - Converted from WordPress PHP template to a modern Rust/WASM architecture
   - Improved security by removing PHP backend dependencies
   - Faster loading times with WebAssembly compilation

2. **UI Enhancements:**
   - Improved color scheme and typography
   - Added hover effects and smooth transitions
   - Modernized card and button designs
   - Enhanced spacing and layout for better readability

3. **Mobile Responsiveness:**
   - Properly optimized for all screen sizes
   - Better handling of navigation menu on small screens
   - Responsive image handling
   - Improved touch target sizes for mobile users

4. **Performance Optimization:**
   - Reduced bundle size with Rust/WASM compilation
   - Optimized animations for better performance
   - Lazy loading of content as the user scrolls

5. **Accessibility:**
   - Improved color contrast
   - Added proper ARIA attributes
   - Enhanced keyboard navigation
   - Screen reader friendly structure

6. **Added Features:**
   - Scroll to top button
   - Improved navigation with smooth scrolling
   - Better organized footer with additional links
   - Enhanced contact section with direct action buttons

## License

[MIT License](LICENSE)
