var searchIndex = JSON.parse('{\
"jb":{"doc":"","t":"AFFFFAAAAAFFFFFFFFFF","n":["cmds","main","setup_logger","cli","dispatch","install","link","list","uninstall","unlink","command","dispatch","command","dispatch","command","dispatch","command","dispatch","command","dispatch"],"q":[[0,"jb"],[3,"jb::cmds"],[10,"jb::cmds::install"],[12,"jb::cmds::link"],[14,"jb::cmds::list"],[16,"jb::cmds::uninstall"],[18,"jb::cmds::unlink"],[20,"clap_builder::parser::matches::arg_matches"],[21,"clap_builder::builder::command"],[22,"core::option"],[23,"jb_lib::error"]],"d":["","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[0,[[],1],[2,1],[[],3],[[[5,[[1,[4,2]]]]],[[6,[1]]]],0,0,0,0,0,[[],3],[2,[[6,[1]]]],[[],3],[2,[[6,[1]]]],[[],3],[2,[[6,[1]]]],[[],3],[2,[[6,[1]]]],[[],3],[2,[[6,[1]]]]],"c":[],"p":[[15,"tuple"],[3,"ArgMatches",20],[3,"Command",21],[15,"str"],[4,"Option",22],[6,"Result",23]],"b":[]},\
"jb_lib":{"doc":"jb-lib","t":"CCAAANENLLLLLLLLLLLLLLLDNNGLLLLLLLLLLLLLLLCDCCLLLLLMLLLLLLLAMLLLLALLLLLLMLLNNNNNNNNNNNNENNNNNNNNNNLLLLLLLLLLLLLLLLLLLLNNNEDLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMLLLMLMLLLLLLLLLLLL","n":["Result","Tool","env","error","tool","ToolsDirectory","Variable","Verbose","borrow","borrow_mut","clone","clone_into","default","env","fmt","from","get","get_or","into","to_owned","try_from","try_into","type_id","Batch","Err","Ok","Result","add","borrow","borrow_mut","errors","fmt","from","from","into","is_empty","len","new","to_string","try_from","try_into","type_id","Kind","Tool","Type","Version","as_path","borrow","borrow_mut","clone","clone_into","directory","download_link","eq","fmt","from","install","into","is_linked","kind","kind","link","list","name","new","release","to_owned","try_from","try_into","type_id","uninstall","unlink","version","with_directory","with_version","Aqua","CLion","ClionNova","DataGrip","DataSpell","DotMemory","DotTrace","Fleet","Gateway","GoLand","IntelliJIdeaCommunity","IntelliJIdeaUltimate","Kind","MPS","PhpStorm","PyCharmCommunity","PyCharmProfessional","Rider","RubyMine","RustRover","Space","WebStorm","Writerside","as_code","as_str","borrow","borrow_mut","clone","clone_into","default_type","eq","fmt","from","into","list","pretty","src_name","to_owned","to_possible_value","try_from","try_into","type_id","value_variants","EAP","Preview","Release","Type","Version","as_str","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","cmp","cmp","compare","compare","compare_builds","default","deserialize","deserialize","eq","eq","equivalent","equivalent","equivalent","equivalent","equivalent","equivalent","fmt","fmt","fmt","from","from","from_str","from_str","into","into","is_latest","major","minor","new","partial_cmp","partial_cmp","patch","pretty","release","to_owned","to_owned","to_possible_value","to_string","try_from","try_from","try_into","try_into","type_id","type_id","value_variants","with_release"],"q":[[0,"jb_lib"],[5,"jb_lib::env"],[23,"jb_lib::error"],[42,"jb_lib::tool"],[75,"jb_lib::tool::kind"],[118,"jb_lib::tool::release"],[178,"alloc::string"],[179,"core::convert"],[180,"core::marker"],[181,"core::marker"],[182,"core::fmt"],[183,"core::any"],[184,"anyhow"],[185,"alloc::vec"],[186,"std::path"],[187,"anyhow"],[188,"clap_builder::builder::possible_value"],[189,"core::cmp"],[190,"serde::de"]],"d":["","","Module for handling defaults and environment variables.","","<code>JetBrains</code> tool management","","","","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","A batch of errors that occurred while executing a command","Contains the error value","Contains the success value","","Add an error to the batch","","","Get the errors in the batch","","Returns the argument unchanged.","Create a new batch of errors from a single error","Calls <code>U::from(self)</code>.","Check if the batch is empty","Get the number of errors in the batch","Create a new batch of errors","","","","","","A JetBrains tool","","","Get the path to the tool.","","","","","The directory the tool is installed to or will be …","Get the download link for the tool.","","","Returns the argument unchanged.","Install the tool.","Calls <code>U::from(self)</code>.","Check if the tool is linked.","The tool kind.","The kind of tool","Link the tool.","List all installed <code>JetBrains</code> tools.","Get the name of the tool.","","Release version parsing and comparison","","","","","Uninstall the tool.","Unlink the tool.","The version of the tool","Set the installation directory of the tool.","Set the version of the tool.","","","","","","","","","","","","","Tool kind","","","","","","","","","","","Returns the code for this tool kind.","Returns the string representation of this tool kind.","","","","","Returns the default release type for this tool kind.","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Returns a list of all tool kinds.","Returns the pretty name for this tool kind.","Returns the source name for this tool kind.","","","","","","","","","","Release type","Release version","Get the release type as a string (e.g. “release”, “…","","","","","","","","","","","","","Compare this version to another version, ignoring the …","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Check if this version is the latest version","","","","","","","Get the release type as a pretty string (e.g. “Release”…","","","","","","","","","","","","","Set the release type"],"i":[0,0,0,0,0,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,29,29,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,0,0,0,0,17,17,17,17,17,17,17,17,17,17,17,17,17,0,17,17,17,17,17,0,17,17,17,17,17,17,17,17,17,21,21,21,21,21,21,21,21,21,21,21,21,0,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,21,23,23,23,0,0,23,23,22,23,22,23,22,23,22,23,22,23,22,22,22,23,22,23,22,23,23,23,22,22,22,23,22,22,23,22,23,22,23,22,22,22,22,22,23,22,22,23,22,23,22,23,22,23,22,23,22,23,22,23,22],"f":[0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[1,1],[[-1,-2],2,[],[]],[1,-1,[[4,[3]],5,6]],[1,7],[[1,8],9],[-1,-1,[]],[1,-1,[[4,[3]],5,6]],[[1,-1],-1,[[4,[3]],5,6]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,11,[]],0,0,0,0,[[12,13],2],[-1,-2,[],[]],[-1,-2,[],[]],[12,[[14,[13]]]],[[12,8],9],[-1,-1,[]],[13,12],[-1,-2,[],[]],[12,15],[12,16],[[],12],[-1,3,[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,11,[]],0,0,0,0,[17,18],[-1,-2,[],[]],[-1,-2,[],[]],[17,17],[[-1,-2],2,[],[]],0,[17,[[19,[0]]]],[[17,17],15],[[17,8],9],[-1,-1,[]],[17,[[19,[2]]]],[-1,-2,[],[]],[17,15],0,0,[17,[[19,[2]]]],[[[20,[18]]],[[19,[[14,[17]]]]]],[17,3],[21,17],0,[-1,-2,[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,11,[]],[17,[[19,[2]]]],[17,[[19,[2]]]],0,[[17,18],17],[[17,22],17],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[21,7],[21,7],[-1,-2,[],[]],[-1,-2,[],[]],[21,21],[[-1,-2],2,[],[]],[21,23],[[21,21],15],[[21,8],9],[-1,-1,[]],[-1,-2,[],[]],[[],[[24,[21]]]],[21,7],[21,7],[-1,-2,[],[]],[21,[[20,[25]]]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,11,[]],[[],[[24,[21]]]],0,0,0,0,0,[23,7],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[23,23],[22,22],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[23,23],26],[[22,22],26],[[-1,-2],26,[],[]],[[-1,-2],26,[],[]],[[22,22],[[19,[26]]]],[[],22],[-1,[[19,[23]]],27],[-1,[[19,[22]]],27],[[23,23],15],[[22,22],15],[[-1,-2],15,[],[]],[[-1,-2],15,[],[]],[[-1,-2],15,[],[]],[[-1,-2],15,[],[]],[[-1,-2],15,[],[]],[[-1,-2],15,[],[]],[[23,8],9],[[22,8],9],[[22,8],9],[-1,-1,[]],[-1,-1,[]],[7,[[19,[23]]]],[7,[[19,[22]]]],[-1,-2,[],[]],[-1,-2,[],[]],[22,15],0,0,[[[20,[28]],[20,[28]],[20,[28]]],22],[[23,23],[[20,[26]]]],[[22,22],[[20,[26]]]],0,[23,7],0,[-1,-2,[],[]],[-1,-2,[],[]],[23,[[20,[25]]]],[-1,3,[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,11,[]],[-1,11,[]],[[],[[24,[23]]]],[[22,23],22]],"c":[],"p":[[4,"Variable",5],[15,"tuple"],[3,"String",178],[8,"From",179],[8,"Send",180],[8,"Sync",180],[15,"str"],[3,"Formatter",181],[6,"Result",181],[4,"Result",182],[3,"TypeId",183],[3,"Batch",23],[3,"Error",184],[3,"Vec",185],[15,"bool"],[15,"usize"],[3,"Tool",42],[3,"PathBuf",186],[6,"Result",184],[4,"Option",187],[4,"Kind",75],[3,"Version",118],[4,"Type",118],[15,"slice"],[3,"PossibleValue",188],[4,"Ordering",189],[8,"Deserializer",190],[15,"u32"],[6,"Result",23]],"b":[[149,"impl-Display-for-Version"],[150,"impl-Debug-for-Version"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
