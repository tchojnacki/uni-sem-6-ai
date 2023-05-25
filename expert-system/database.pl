% swipl -s database.pl
% ?- main.

%%% PART DEFINITIONS %%%

% def_part(Part, Label, Tags, Parent).
def_part(printer, "Printer", [], null).
def_part(internal_battery, "Internal Battery", [battery], printer).
def_part(camera, "Camera", [], printer).
def_part(camera_battery, "Camera Battery", [battery], printer).
def_part(control_panel, "Control Panel", [], printer).
def_part(save_button, "Save Button", [button], control_panel).
def_part(control_button, "Control Button", [button], control_panel).
def_part(camera_button, "Camera Button", [button], control_panel).

is_part(Part) :- def_part(Part, _, _, _).
part_parent(Part, Parent) :-
    is_part(Parent),
    def_part(Part, _, _, Parent).
part_tag(Part, Tag) :-
    def_part(Part, _, Tags, _),
    member(Tag, Tags).

%%% PROBLEM CATEGORY DEFINITIONS %%%

% def_category(Category, Label).
def_category(hardware, "Hardware Problems").
def_category(printing, "Printing Problems").
def_category(bluetooth, "Bluetooth Problems").
def_category(errors, "Error Message Shown").

is_category(Category) :- def_category(Category, _).

%%% PROBLEM DEFINITIONS %%%

% def_problem(Problem, Label, Category).

% HARDWARE
def_problem(light_flashing_red, "The Status light is flashing red.", hardware).
def_problem(flash_after_off, "The On light flashed briefly after I turned the printer off.", hardware).
def_problem(images_not_found, "The printer does not find and display the images.", hardware).
def_problem(mac_displays_zero_images, "When the printer is connected to a Mac, iPhoto displays 0 images in the camera.", hardware).
def_problem(not_all_images_transferred, "Not all the images on camera were transferred to a computer.", hardware).
def_problem(will_not_turn_on, "The printer will not turn on.", hardware).
def_problem(printer_makes_noises, "The printer makes noises.", hardware).
def_problem(remote_not_working, "The remote does not work.", hardware).
def_problem(camera_not_recognized, "The printer does not recognize the camera.", hardware).
def_problem(nothing_on_tv, "Nothing is displayed on television.", hardware).
def_problem(not_responding(Button), Label, hardware) :-
    part_tag(Button, button),
    label(Button, L),
    string_concat(L, " on the control panel does not respond.", Label).
def_problem(not_charging(Battery), Label, hardware) :-
    part_tag(Battery, battery),
    label(Battery, L),
    string_concat(L, " will not charge.", Label).

% PRINTING
def_problem(paper_not_feeding, "Printer does not feed into the printer correctly.", printing).
def_problem(image_at_angle, "The image is printed at an angle or is off-center.", printing).
def_problem(no_page_out, "No page came out of the printer.", printing).
def_problem(paper_jammed, "The paper jammed while printing.", printing).
def_problem(blank_page_out, "A blank page came out of the printer.", printing).
def_problem(printer_ejects_paper, "The printer ejects the paper when preparing to print.", printing).
def_problem(print_quality_poor, "Print quality is poor.", printing).
def_problem(images_marked_do_not_print, "The images marked for printing in the digital camera do not print.", printing).
def_problem(commands_dimmed, "Some commands in the menus are dimmed.", printing).

% BLUETOOTH
def_problem(cannot_find_printer, "My Bluetooth device cannot find the printer.", bluetooth).
def_problem(image_printed_with_borders, "The image printed with borders.", bluetooth).

% ERRORS
def_problem(error_open_output, "Open output door or clear paper jam, then press OK.", errors).
def_problem(error_incompatible_cartridge, "Print cartridge is not compatible. Use appropriate cartridge.", errors).
def_problem(error_camera_connected, "Already connected to a camera.", errors).
def_problem(error_computer_connection, "Check computer connection.", errors).

% HELPERS
is_problem(Problem) :- def_problem(Problem, _, _).
problem_category(Problem, Category) :-
    is_category(Category),
    def_problem(Problem, _, Category).

%%% FIXES %%%
step(turn_off) :-
    ask("Is the internal battery installed in the printer?"),
    write("- Remove the battery."), nl,
    write("- Wait about 10 seconds."), nl,
    write("- Reinstall the battery.").

step(turn_off) :-
    write("- Unplug the power cord."), nl,
    write("- Wait about 10 seconds."), nl,
    write("- Plug the power cord back in.").

fix(light_flashing_red) :-
    ask("Is the camera connected?"),
    write("Check camera screen for instructions.").

fix(light_flashing_red) :-
    ask("Is the printer connected to a computer?"),
    write("Check the computer monitor.").

fix(light_flashing_red) :- step(turn_off).

fix(flash_after_off) :-
    write("This is a normal part of the power-down process."), nl, 
    write("It does not indicate a problem with the printer.").

fix(_) :- write("No suitable fix found :(").

%%% MENU HELPERS %%%

label(Part, Label) :- def_part(Part, Label, _, _).
label(Category, Label) :- def_category(Category, Label).
label(Problem, Label) :- def_problem(Problem, Label, _).

print_list(List) :- print_list(List, 1).
print_list([], _).
print_list([Head | Tail], N) :-
    write(N), write(". "), write(Head), nl,
    N1 is N + 1,
    print_list(Tail, N1).

select_menu(Name, List, Result) :-
    write("Choose "), write(Name), write(":"), nl,
    maplist(label, List, Labels),
    print_list(Labels),
    write("Number: "), read(Number),
    nth1(Number, List, Result),
    label(Result, ResultLabel),
    write("Selected "), write(Name), write(": "), write(ResultLabel), nl.

is_truthy('y').
is_truthy('Y').
is_truthy('yes').
is_truthy('Yes').
is_truthy('YES').

ask(Question) :-
    write(Question), write(" [y/n]: "), read(Decision),
    is_truthy(Decision).

%%% MAIN PROCEDURE %%%

main :-
    prompt(_, ''),
    findall(C, is_category(C), Categories),
    select_menu("problem category", Categories, Category),
    findall(P, problem_category(P, Category), Problems),
    select_menu("problem", Problems, Problem),
    fix(Problem), nl.
