using Random

struct LiaDocInfo
    initialsed::Bool
    file_name::String
    output_folder_canonical::String
    image_folder_canonical::String
end

doc_info = LiaDocInfo(false, "", "", "");

# Must be called when session is initialised
if doc_info.initialsed
    error("Julia session already initialised for this document.");
end
doc_info.file_name = file_name
doc_info.initialsed = true


function embed_fig(fig::Figure, width::Float64)
    if !doc_info.initialsed
        error("Julia session not initialised for this document.");
    end
    file_name = randstring(12) * ".png";
    img_output_path = joinpath(doc_info.image_folder_canonical, file_name);
    savefig(fig, img_output_path);
    # TODO: Make this work with other formats and LaTeX image packages
    return "\\includegraphics[width=", width, "\\textwidth]{", file_name, "}";
end