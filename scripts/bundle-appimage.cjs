const path = require('path');

// Require bundler
const { Bundler } = require('neutralino-appimage-bundler');

// Create an object with some params
const bundler = new Bundler({
    // .desktop file properties
    desktop: {
        // Name field
        name: 'Honkers Launcher',

        // Path to the icon
        icon: path.join(__dirname, '../public/icons/64x64.png'),

        // Categories (defult is Utilities)
        categories: ['Game']
    },

    // Neutralino binary info
    binary: {
        // Name of the binary (cli.binaryName)
        name: 'honkers-launcher',

        // Dist folder path
        dist: path.join(__dirname, '../dist')
    },

    // Should AppImage contain Neutralino's dependencies or not
    // If true, then AppImage will contain binary's shared libraries
    includeLibraries: false,

    // Some files or folders to copy inside of the the AppImage
    copy: {
        'public': path.join(__dirname, '../dist/honkers-launcher/public')
    },

    // Path to the appimage to save
    output: path.join(__dirname, '../dist/honkers-launcher-1.0.0.AppImage'),

    // Application version
    version: '1.0.0'
});

// Bundle project
bundler.bundle();
