%%% PROBLEM CATEGORY DEFINITIONS %%%

def_category(hardware, 'Printer Hardware Problems').
def_category(printing, 'Printing Problems').
def_category(bluetooth, 'Bluetooth Printing Problems').
def_category(errors, 'Error Message Shown').

%%% PROBLEM DEFINITIONS %%%

% HARDWARE
def_problem(light_flashing_red, hardware, 'The Status light is flashing red.').
def_problem(flash_after_off, hardware, 'The On light flashed briefly after I turned the printer off.').
def_problem(control_buttons_not_responding, hardware, 'The buttons on the control panel do not respond.').
def_problem(save_button_not_responding, hardware, 'The Save button on the control panel does not respond.').
def_problem(images_not_found, hardware, 'The printer does not find and display the images.').
def_problem(mac_displays_zero_images, hardware, 'When the printer is connected to a Mac, iPhoto display 0 images in the camera.').
def_problem(not_all_images_transferred, hardware, 'Not all the images on camera were transferred to a computer.').
def_problem(will_not_turn_on, hardware, 'The printer will not turn on.').
def_problem(printer_will_not_charge, hardware, 'The printer will not charge.').
def_problem(printer_makes_noises, hardware, 'The printer makes noises.').
def_problem(remote_not_working, hardware, 'The remote does not work.').
def_problem(camera_buttons_not_working, hardware, 'The camera buttons are not working.').
def_problem(pict_bridge_not_recognized, hardware, 'The printer does not recognize the PictBrudge camera.').
def_problem(nothing_on_tv, hardware, 'Nothing is displayed on televosion.').
def_problem(camera_will_not_charge, hardware, 'The camera batteries will not charge.').

% PRINTING
def_problem(paper_not_feeding, printing, 'Printer does not feed into the printer correctly.').
def_problem(image_at_angle, printing, 'The image is printed at an angle or is off-center.').
def_problem(no_page_out, printing, 'No page came out of the printer.').
def_problem(paper_jammed, printing, 'The paper jammed while printing.').
def_problem(blank_page_out, printing, 'A blank page came out of the printer.').
def_problem(printer_ejects_paper, printing, 'The printer ejects the paper when preparing to print.').
def_problem(print_quality_poor, printing, 'Print quality is poor.').
def_problem(images_marked_do_not_print, printing, 'The images marked for printing in the digital camera do not print.').
def_problem(commands_dimmed, printing, 'Some commands in the menus are dimmed.').

% BLUETOOTH
def_problem(cannot_find_printer, bluetooth, 'My Bluetooth device cannot find the printer.').
def_problem(image_printed_with_borders, bluetooth, 'The image printed with borders.').

% ERRORS
def_problem(error_open_output, errors, 'Open output door or clear paper jam, then press OK.').
def_problem(error_incompatible_cartridge, errors, 'Print cartridge is not compatible. Use appropriate cartridge.').
def_problem(error_camera_connected, errors, 'Already connected to a camera.').
def_problem(error_computer_connection, errors, 'Check computer connection.').

%%% BASIC HELPER PREDICATES %%%

is_category(C) :- def_category(C, _).
is_problem(P) :- def_problem(P, _, _).
problem_category(P, C) :- is_problem(P), is_category(C), def_problem(P, C, _).

%%% HUMAN-READABLE LABELS %%%

label(C, L) :- is_category(C), def_category(C, L).
label(P, L) :- is_problem(P), def_problem(P, _, L).
