# What is this?  Why doesn't it compile?

This is a project I worked on for a week or two while learning Rust. I really
like Rust and want to get back to working on this and learning it. Right now the
code doesn't even build, I've just pushed the state of the tree when I abandoned
it, probably including a lot of unnecessary files.

Here is the idea for the project:

1. When the user presses some hotkey, take a screenshot. Some preprocessing is
   done immediately.

2. The user then presses a key that corresponds to some character on the screen.

3. This program will attempt to quickly locate instances of that character on
   the screen, and display an overlay at each location.

4. These overlays will correspond to the keys that should be pressed in order to
   click on that location.

The idea for this comes from using tools like ace jump mode in emacs, and also
browser plugins like vimium / vimperator / pentadactyl / etc. It is also
inspired by the 'keynav' project. The goal is to be able to use these sort of
quick keyboard interactions, but with any application. I particularly want to
use this to quickly select and copy a region of text, such as in a browser
window.

OCRing the entire screenshot would probably involve far too much processing.
Since this is a tool intended for quick keyboard interaction, latency must be
minimized as much as possible.  The key idea here is that it should be much
faster to find a particular character.

If I recall correctly, the current code is still pretty inefficient. It renders
the particular character and then uses some inefficiently implemented
convolution to find locations that likely have the character. Doing this
efficiently with scale invariance is tricky, and will need something more
sophisticated than rendering the character at different sizes and using
convolution.  It will likely require some sort of feature detection algorithm
which is pre-trained on particular fonts / characters.

The name `ainav` comes from me wanting a hobby project involving machine
learning. I figured some machine learning techniques like decision trees might
end up being involved. Certainly computer vision techniques would be used.
