#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <X11/XKBlib.h>
#include <X11/extensions/XKBrules.h>


/// https://unix.stackexchange.com/a/422493/162189
/// Requires libxkbfile-dev
/// gcc getxkblayout.c -lX11 -lxkbfile -o ~/bin/getxkblayout
int main() {
  Display *dpy = XOpenDisplay(NULL);
  if(dpy == NULL) {
    fprintf(stderr, "Cannot open display\n");
    return 1;
  }

  XkbStateRec state;
  XkbGetState(dpy, XkbUseCoreKbd, &state);

  XkbRF_VarDefsRec vd;
  XkbRF_GetNamesProp(dpy, NULL, &vd);

  char *tok = strtok(vd.layout, ",");
  for(int i = 0; i < state.group; i++) {
    tok = strtok(NULL, ",");
    if (tok == NULL) {
      return 1;
    }
  }

  printf("%s", tok);

  return 0;
}
