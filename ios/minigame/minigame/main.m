//
//  main.m
//  minigame
//
//  Created by Valerio Santinelli on 06/06/17.
//  Copyright © 2017 Valerio Santinelli. All rights reserved.
//

#import <UIKit/UIKit.h>
#import "AppDelegate.h"

/*
extern void run_loop();

int main(int argc, char * argv[]) {
  @autoreleasepool {
      return UIApplicationMain(argc, argv, nil, NSStringFromClass([AppDelegate class]));
  }
}
 */

extern void SDL_main();

int main(int argc, char * argv[]) {
    SDL_main();
}
